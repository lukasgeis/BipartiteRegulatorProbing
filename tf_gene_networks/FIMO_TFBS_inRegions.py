import argparse

"""Takes a bed file with regions and runs Fimo on them. Afterwards converts Fimo's output into a matrix of regions x TFs
 with the counted binding sites. Only regions with at least one binding site are written to output. 
 Overlapping TFBS of the same TF on the same strand are merged and counted as 1. 
 Includes the steps of writing the sequence of the regions, forcing the regions to be within the genome boundaries, 
 and adjusting the TF motif meme-file so that the base content of the bed-regions is used as background frequencies.
 CARE: Tested for meme5.4.1, later versions handle sequence names differently."""

parser = argparse.ArgumentParser()
parser.add_argument("--bed_file", required=True, help='Bed-file with regions for which TFBS should be annotated.')
parser.add_argument("--PWMs", required=True, help='TF motif file in meme format.')
parser.add_argument("--fasta", required=True, help='Genome sequence file in fasta format.')
parser.add_argument("--fimo_src", required=True, help='Path to the Fimo executable. If fimo is on path, just type "fimo".')
parser.add_argument("--out_dir", required=True, help='Path to which to write the output to.')
parser.add_argument("--write_sequence", default='False', help='If Fimo should write the sequence matched '
                                                                       'to the motif in its output file [True, False].')


if __name__ == '__main__':
    # This is ugly and a style violation, but otherwise can't convince the Sphinx markdown to document this function.
    from pybedtools import BedTool
    import FIMO_TFBS_Helper

    args, seq_out, fimo_out = FIMO_TFBS_Helper.process_args(parser.parse_args())

    # Limit the bed file to the coordinates, otherwise there can be weird stuff happening.
    inter_bed = BedTool('\n'.join(['\t'.join(x.split('\t')[:3]) for x in open(args.bed_file).readlines() if not x.startswith('#')]),
                        from_string=True)
    # Force the regions to be within the chromosome boundaries.
    inter_bed = inter_bed.slop(g=args.fasta + '.fai', b=0)
    inter_bed = BedTool(''.join([str(x) for x in inter_bed if x.length > 1]), from_string=True)
    if not inter_bed:
        print("No remaining region after capping at locations covered by the fasta file.")
        exit()
    inter_bed.sequence(fi=args.fasta, fo=seq_out)
    print('sequence fasta stored at', seq_out)

    new_meme_file = FIMO_TFBS_Helper.meme_fitbackground(args.PWMs, seq_out, args.out_dir)

    FIMO_TFBS_Helper.fimo_runner(args, seq_out, fimo_out, new_meme_file)

    FIMO_TFBS_Helper.fimo_processor(args, fimo_out)


