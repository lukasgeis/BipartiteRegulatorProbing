import subprocess
import os
import sys
import datetime
import argparse
import pandas as pd
import numpy as np
import scipy.stats
import json
from multiprocessing import Pool
from timeit import default_timer as clock
import Format_MEME
import Process_GTF

"""Write a matrix of TFs*Genes for a given sample based on TFBS predicted in enhancers by FIMO.
The entries in the matrix are the fraction of REs that contain a motif of a TF. The entries are set to 0 if
there is no enrichment of REs with a TFBS among all REs of a gene. The matrix is restricted to TFs that have
a peak at any of their promoter and to all genes in the annotation."""

script_path = os.path.realpath(__file__)  # Needed later to find another script with subprocess.

parser = argparse.ArgumentParser()
parser.add_argument('json_file')
args = parser.parse_args()

input_dict = json.load(open(args.json_file))

if not os.path.isdir(input_dict['output_path']):
    os.mkdir(input_dict['output_path'])
else:
    print("WARNING: output_path already exists, will write files into it.")

fimo_out = os.path.join(input_dict['output_path'], "FimoRuns/")
if not os.path.isdir(fimo_out):
    os.mkdir(fimo_out)

input_dict['start time'] = str(datetime.datetime.now())
json.dump(input_dict, open(os.path.join(input_dict['output_path'], args.json_file.split('/')[-1]), 'w'))

# ------------------------------------------------------------------------------------------
# Call the Fimo function
# ------------------------------------------------------------------------------------------
# Get a mapping of the TF names in the meme file.
motif_id_map, all_tf_names, missed_ids = Format_MEME.meme_id_map(meme_file=input_dict['pwm_file'],
                                                                 gtf_file=input_dict['gtf_file'], species='human')

if str(input_dict['open_tfs_only']).lower() == 'true':
    # Find the genes which have one of the peaks in their promoter.
    open_promoter = Process_GTF.gene_window_bed(gtf_file=input_dict['gtf_file'], extend=200, tss_type='all', dict_only=False,
                                                   merge=True, open_regions=input_dict['peak_file'])
    open_genes = set([x.fields[3] for x in open_promoter])
    # And find the motifs whose TFs are all among the open promoter.
    keep_tfs = [x for x in motif_id_map.keys() if np.all([s in open_genes for s in motif_id_map[x]])]
else:
    keep_tfs = list(motif_id_map.keys())

fimo_out_file = os.path.join(fimo_out, "Out_Fimo_TFBSMatrix.txt.gz")  # The name is fixed by the function below.
if not os.path.isfile(fimo_out_file):
    reduced_pwm_file = os.path.join(fimo_out, "OpenTFs_meme.txt")
    Format_MEME.subset_meme(meme_file=input_dict["pwm_file"], motif_names=keep_tfs,
                                out_file=reduced_pwm_file, include_dimers=False, exact_match=True)
    fimo_python_script = os.path.join(os.path.dirname(script_path), "FIMO_TFBS_inRegions.py")
    fimo_region_cmd = "python3 " + fimo_python_script + " --bed_file " + \
                        input_dict['peak_file'] + " --PWMs " + reduced_pwm_file + " --fasta " + input_dict['fasta_file'] + \
                        " --fimo_src " + input_dict['fimo_src'] + " --out_dir " + fimo_out+'/'
    subprocess.call(fimo_region_cmd, shell=True)
else:
    print("Using existing FIMO output:", fimo_out_file)

# ------------------------------------------------------------------------------------------
# Process TFBS
# ------------------------------------------------------------------------------------------
tfbs_df = pd.read_table(fimo_out_file, sep='\t', header=0).set_index('region')
if tfbs_df.empty:
    print("ERROR: No FIMO hits found in the regions")
    sys.exit()
tfbs_df.index = [i.replace('chr', '') for i in tfbs_df.index]

# Get the gABC interaction file and regions per gene.
abc_inter_df = pd.read_table(input_dict['abc_interactions'], sep='\t', header=0)
gene_region_map = {g.split('.')[0]: set() for g in set(abc_inter_df['Ensembl ID'])}
for entry in abc_inter_df.to_dict(orient='records'):
    gene_region_map[entry['Ensembl ID'].split('.')[0]].add(entry['PeakID'])

# Per TF get the number of regions that form interactions with TFBS (binary).
tfbs_df = tfbs_df.loc[[x for x in set(abc_inter_df['PeakID']) if x in tfbs_df.index]]
tf_num_regions = {tf: (tfbs_df[tf] > 0).sum() for tf in tfbs_df.columns}
# Store per region which TFs have ≥1 binding sites. For regions with ≥1 interaction, even if there are no TFBS.
region_tf_map = {r: set() if r not in tfbs_df.index else set(tfbs_df.columns[tfbs_df.loc[r] > 0]) for r in set(abc_inter_df['PeakID'])}


def get_tf_gene_weight(args):
    """
    For each gene get the edge weights to the TFs.
    """
    gene, cre_tf_map, tf_num_cres = args
    regions = gene_region_map[gene]
    gene_tfs = set().union(*[cre_tf_map[r] for r in regions])
    regions_tf_counts = {t: len([r for r in regions if t in cre_tf_map[r]]) for t in gene_tfs}
    tf_vals = {}
    for tf in gene_tfs:
        fisher_table = [[regions_tf_counts[tf], len(regions) - regions_tf_counts[tf]], 
                        [tf_num_cres[tf] - regions_tf_counts[tf],  len(cre_tf_map) - tf_num_cres[tf] - (len(regions) - regions_tf_counts[tf])]]
        _, pval = scipy.stats.fisher_exact(fisher_table, alternative='greater')

        val = regions_tf_counts[tf] / len(regions) if pval <= 0.05 else 0

        tf_vals[tf] = val
    return [gene, tf_vals]


start_corr = clock()
process_pool = Pool(processes=input_dict['cores'])
pool_vals = process_pool.map(get_tf_gene_weight, [[g, region_tf_map, tf_num_regions] for g in list(gene_region_map.keys())])
process_pool.close()
print(clock() - start_corr)

# ------------------------------------------------------------------------------------------
# Write output and compress
# ------------------------------------------------------------------------------------------
gene_tf_net_out = os.path.join(input_dict['output_path'], "TFGeneNet.txt")
with open(gene_tf_net_out, 'w') as net_out:
    net_out.write('Ensembl ID\t#CREs\t' + '\t'.join(tfbs_df.columns) + '\n')
    for g, val_dict in pool_vals:
        net_out.write(g + '\t' + str(len(gene_region_map[g])) + '\t' + '\t'.join(['0' if tf not in val_dict else str(val_dict[tf]) for tf in tfbs_df.columns]) + '\n')

subprocess.call('gzip -f ' + gene_tf_net_out, shell=True)


