from itertools import chain
import Process_GTF

"""Collection of functions related to formatting or subsetting TF motif files in meme-format."""


def meme_monomer_map(meme_file):
    """
    Takes a motif file in meme format and creates a map of the motif name to a list of the constituent monomers,
    removing any motif versions in the list values. E.g. {'FOS(MA1951.1)': ['FOS'], 'FOXJ2::ELF1': ['FOXJ2', 'ELF1']}.

    Returns:
        tuple:
            - **tf_monomer_map**: A dictionary with {motif name: List of TF names}
            - **all_monomer_names**: List of all the monomer names, meaning a list of all unique values of tf_monomer_map.
    """
    tfs = [x.split('\n\n')[0].split(' ')[0] for x in open(meme_file).read().split('MOTIF ')[1:]]
    tf_monomer_map = {t: [x.split('(')[0] for x in t.split('::')] for t in tfs}
    all_monomer_names = list(set(chain(*tf_monomer_map.values())))
    return tf_monomer_map, all_monomer_names


def meme_id_map(meme_file, gtf_file, species='human'):
    """
    Takes a motif meme-file and returns the list of TF gene ids belonging to that TF. Note that it assumes a certain
    syntax for the different motif versions. Species is required for looking up names missing in the
    gtf file with the MyGene.info API.

    Args:
        meme_file: Motif file in meme format.
        gtf_file: gtf-file in GENCODE's format, can be gzipped.
        species: 'human' or 'mouse'. Others are not tested.

    Returns:
        tuple:
            - **tf_ids**: Dictionary of {motif: [Ensembl IDs]} with an ID of each constituent monomer.
            - **all_tf_names**: List of all monomers without any version.
            - **misses**: Motif names that could not be mapped.
    """
    tf_monomer_map, all_tf_names = meme_monomer_map(meme_file)
    tf_ids = {t: [] for t in tf_monomer_map.keys()}
    misses = []
    mapped_names, missed_names = Process_GTF.match_gene_identifiers(all_tf_names, gtf_file=gtf_file, species=species,
                                                                       scopes="symbol", fields="ensembl")
    for tf in tf_ids:
        sub_tfs = tf_monomer_map[tf]
        for sub in sub_tfs:
            if sub in mapped_names:
                tf_ids[tf].append(mapped_names[sub]['ensembl'])
            else:
                misses.append(tf)
                print(sub, 'name not mappable')
    # Kick out motif names with non-mappable monomers.
    tf_ids = {k: val for k, val in tf_ids.items() if k not in misses}
    return tf_ids, all_tf_names, misses


def subset_meme(meme_file, motif_names, out_file, include_dimers=True, exact_match=False):
    """
    Takes a meme file and writes a new one containing only the ones present in motif_names. Headerlines are preserved.

    Args:
        meme_file: Motif file in meme format.
        motif_names: List of motif names to keep for the new file.
        out_file: Path where to write the new subsetted meme file.
        include_dimers: Also adds dimers containing one of the motif_names.
        exact_match: If motifs have to be exact matches to the motif_names. If False, allows version suffixes, e.g. BHLHA15(MA0607.2).
    """

    meme = open(meme_file).read()
    header_block = meme.split("\nMOTIF")[0]
    tf_blocks = meme.split('MOTIF ')[1:]

    with open(out_file, 'w') as output:
        output.write(header_block + '\n')
        for block in tf_blocks:
            if not exact_match:
                this_tf = block.split('\n\n')[0].split(' ')[0].split('(')[0]
            else:
                this_tf = block.split('\n\n')[0].strip().split(" ")[0]
            if include_dimers:  # If any matches.
                if sum([sub_t in motif_names for sub_t in this_tf.split('::')]) > 0 or this_tf in motif_names:
                    output.write("MOTIF " + block)
            else:
                if this_tf in motif_names:  # If all match.
                    output.write("MOTIF " + block)
