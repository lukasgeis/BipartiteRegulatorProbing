from collections import Counter
import subprocess
import os
import gzip
from pybedtools import BedTool

"""Helper functions for calling FIMO and processing its output."""


def process_args(args):
    if args.write_sequence.lower() == 'true':
        args.write_sequence = True
    elif args.write_sequence.lower() == 'false':
        args.write_sequence = False
    else:
        print("ERROR unrecognized option for write_sequence:", args.write_sequence)

    if not args.out_dir.endswith('/'):
        args.out_dir += '/'

    if not os.path.isdir(args.out_dir):
        os.mkdir(args.out_dir)

    if not os.path.isfile(args.fasta + '.fai'):
        print("Genome file missing, creating with samtools faidx")
        subprocess.call('samtools faidx ' + args.fasta, shell=True)

    seq_out = args.out_dir + '/Sequences.fa'
    fimo_out = args.out_dir + 'Out_Fimo.tsv.gz'
    return args, seq_out, fimo_out


def meme_fitbackground(meme_file, sequence_file, out_dir):
    """
    Takes a TF motif file in meme format and a fasta sequence file to create a new meme file where the
    base content in the header fits to the frequencies in the sequence file.
    """
    base_occs = Counter(
        ''.join([x.strip().lower().replace('n', '') for x in open(sequence_file).readlines() if not x.startswith('>')]))
    cg_content = (base_occs['c'] + base_occs['g']) / sum(base_occs.values()) / 2
    at_content = (base_occs['a'] + base_occs['t']) / sum(base_occs.values()) / 2

    meme = open(meme_file).read()
    new_meme_file = out_dir + meme_file.split('/')[-1].split('.')[0] + "_bg.txt"
    meme = [x for x in meme.split('\n\n') if not x.startswith('Background')]
    new_background = 'Background letter frequencies:\nA ' + str(round(at_content, 5)) + ' C ' + str(round(
        cg_content, 5)) + ' G ' + str(round(cg_content, 5)) + ' T ' + str(round(at_content, 5))
    new_meme = meme[:3] + [new_background] + meme[3:]
    open(new_meme_file, 'w').write('\n\n'.join(new_meme))
    print('new meme file with matching background at', new_meme_file)
    return new_meme_file


def fimo_runner(args, seq_out, fimo_out, new_meme_file):
    """
    A wrapper to call FIMO to avoid redundant code for catching specific flags.
    """
    print("Running Fimo")
    write_seq = '' if args.write_sequence else '--skip-matched-sequence'
    # FIMO has its own names when writing the sequence which makes it annoying to handle.
    out_suffix = '--oc '+args.out_dir if args.write_sequence else ""
    std_out_suffix = '' if args.write_sequence else " > " + fimo_out.replace('.gz', '')

    bashCommand = args.fimo_src + " --thresh 0.0001 " + out_suffix + ' ' + write_seq + " --verbosity 1 --bfile --motif-- " + new_meme_file \
                  + " " + seq_out + std_out_suffix
    subprocess.call(bashCommand, shell=True)

    if args.write_sequence:
        os.rename(args.out_dir + 'fimo.tsv', fimo_out.replace('.gz', ''))
    subprocess.call('gzip ' + fimo_out.replace('.gz', ''), shell=True)
    print('fimo output at', fimo_out)


def fimo_processor(args, fimo_out):
    """
    Takes a FIMO output file and creates a matrix of seq_names to TFBS, while merging all TFBS that are in the
    same region on the strand and counting them as 1.
    """
    print("Processing Fimo output")
    fimo_fetcher = ''  # Create a bed object to merge hits, with the chromosome as TF#region#strand
    all_regions = set()
    with gzip.open(fimo_out, 'rt') as fimo_in:
        fimo_head = {x: i for i, x in enumerate(fimo_in.readline().strip().split('\t'))}
        for row in fimo_in:
            if not row.startswith('#'):
                entry = row.strip().split('\t')
                if args.write_sequence and len(entry) != len(fimo_head):
                    continue
                # The naming scheme of the sequences in the fasta file might differ.
                if entry[fimo_head['sequence_name']].startswith('::'):
                    seq_region = entry[fimo_head['sequence_name']].split('::')[1]
                else:
                    seq_region = entry[fimo_head['sequence_name']].split('::')[0]
                all_regions.add(seq_region)
                fimo_fetcher += entry[fimo_head['motif_id']] + '#' + seq_region + '#' + \
                                entry[fimo_head['strand']] + '\t' + entry[fimo_head['start']] + '\t' + entry[fimo_head['stop']] + '\n'
    # The merging is done on TF#peakstring#strand | TFBS start | TFBS end, so strand is considered in the merge.
    fimo_bed = BedTool(fimo_fetcher, from_string=True).sort().merge()

    tfs = [x.split('\n\n')[0].split(' ')[0] for x in open(args.PWMs).read().split('MOTIF ')[1:]]
    region_hits = {e: {tf: 0 for tf in tfs} for e in all_regions}

    for hit in fimo_bed:
        region_hits[hit.fields[0].split('#')[1]][hit.fields[0].split('#')[0]] += 1

    with open(fimo_out.replace('Fimo.tsv.gz', 'Fimo_TFBSMatrix.txt'), 'w') as output:
        output.write('region\t' + '\t'.join(tfs) + '\n')
        for region, vals in region_hits.items():
            output.write(region + '\t' + '\t'.join([str(vals[tf]) for tf in tfs]) + '\n')

    fimo_count_out = fimo_out.replace('Fimo.tsv.gz', 'Fimo_TFBSMatrix.txt')
    subprocess.call('gzip ' + fimo_count_out, shell=True)
    print("FIMO hits counted")
    print(fimo_count_out + '.gz')


