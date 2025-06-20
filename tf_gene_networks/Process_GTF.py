from pybedtools import BedTool
import gzip
from itertools import chain
import requests
import time
import math


def gene_window_bed(gtf_file, extend=200, gene_set=set(), tss_type='5', dict_only=False, merge=False,
                    open_regions=False):
    """
    Based on a gtf file fetches all or the most 5' TSS for all genes, and returns a BedTool object with windows
    around the TSS, expanding by 'extend' in each direction, resulting in a total window size of 2*'extend'+1.
    Alternatively gives a dictionary with the TSS, also containing the number of transcripts, gene name and strand.
    The BedTools intervals will be 0-based, the TSS in the dictionary still 1-based like in the gtf-file.
    Care: removes the .-suffixes from all gene IDs.

    Args:
        gtf_file: gtf-file in GENCODE's format, can be gzipped.
        extend: Number of base pairs to extend the TSS in each direction. 200 means a window of size 401.
        gene_set: Set of Ensembl IDs or gene names or mix of both to limit the output to. If empty, return for all
            genes in the annotation.
        tss_type: "5" to get only the 5' TSS or "all" to get all unique TSS of all transcripts in the gtf-file.
        dict_only: Returns a dictionary instead of a BedTool's object.
        merge: If True, merges all overlapping promoters of the same gene into one row in the BedTool's object.
        open_regions: Optional bed file or BedTools' object, only overlapping parts of promoters will be kept for the
            BedTool's object. Can therefore be used to find genes whose promoter overlap a set of peaks, for example to
            find genes that are accessible.
    """
    if tss_type == '5':
        identifier = 'gene'
    elif tss_type == 'all':
        identifier = 'transcript'
    if gtf_file.endswith('.gz'):
        file_opener = gzip.open(gtf_file, 'rt')
    else:
        file_opener = open(gtf_file)

    if gene_set:
        gene_set = set([g.split('.')[0] for g in gene_set])

    tss_locs = {}
    with file_opener as gtf_in:
        for entry in gtf_in:
            if not entry.startswith('#') and entry.split('\t')[2] == identifier:
                line = entry.strip().split('\t')
                # Some gene IDs are non-unique if they have a _PAR_Y version.
                if not line[8].split('gene_id "')[-1].split('";')[0].endswith("_PAR_Y"):
                    this_gene = line[8].split('gene_id "')[-1].split('";')[0].split('.')[0]
                    gene_name = line[8].split('gene_name "')[-1].split('";')[0]
                    if not gene_set or this_gene in gene_set or gene_name in gene_set:
                        if this_gene not in tss_locs:
                            tss_locs[this_gene] = {'chr': None, 'tss': set(), '#transcripts': 0}

                        tss_locs[this_gene]['chr'] = line[0]
                        tss_locs[this_gene]['name'] = gene_name
                        if line[6] == '+':
                            if identifier == 'gene' and (not tss_locs[this_gene]['tss'] or list(tss_locs[this_gene]['tss'])[0] > int(line[3])):
                                tss_locs[this_gene]['tss'] = {int(line[3])}
                            elif identifier == 'transcript':
                                tss_locs[this_gene]['tss'].add(int(line[3]))
                                tss_locs[this_gene]['#transcripts'] += 1
                            tss_locs[this_gene]['strand'] = '+'
                        if line[6] == '-':
                            if identifier == 'gene' and (not tss_locs[this_gene]['tss'] or list(tss_locs[this_gene]['tss'])[0] < int(line[4])):
                                tss_locs[this_gene]['tss'] = {int(line[4])}
                            elif identifier == 'transcript':
                                tss_locs[this_gene]['tss'].add(int(line[4]))
                                tss_locs[this_gene]['#transcripts'] += 1
                            tss_locs[this_gene]['strand'] = '-'

    if dict_only:
        return tss_locs

    promoter_bed = BedTool('\n'.join(chain(*[[vals['chr'] + '\t' + str(max([0, tss - int(extend) - 1])) + '\t' +
                                             str(tss + int(extend)) + '\t' + g + '\t.\t' + vals['strand'] for tss in vals['tss']]
                                             for g, vals in tss_locs.items()])), from_string=True)

    if open_regions and str(open_regions).lower() != "false":
        if type(open_regions) == str:
            open_regions = BedTool('\n'.join(['\t'.join(x.strip().split('\t')[:3]) for x
                                              in open(open_regions).readlines() if not x.startswith('#')]), from_string=True)
        promoter_bed = promoter_bed.intersect(open_regions)

    if merge:  # Flip the chr and Ensembl ID column to merge promoter of the same gene, and afterwards flip again.
        # Due to using the Ensembl ID as the first coordinate we don't need to consider the strand for merging (relying
        # on the annotations putting a gene only on one strand exclusively).
        promoter_bed = BedTool('\n'.join(['\t'.join([x.fields[3], x.fields[1], x.fields[2], x.fields[0], x.fields[4], x.fields[5]]) for x in promoter_bed]), from_string=True).sort().merge(c=[4, 5, 6], o='distinct')
        promoter_bed = BedTool('\n'.join(['\t'.join([x.fields[3], x.fields[1], x.fields[2], x.fields[0], x.fields[4], x.fields[5]]) for x in promoter_bed]), from_string=True)

    return promoter_bed


def match_gene_identifiers(gene_identifiers, gtf_file='', species='human', scopes="symbol,alias,uniprot",
                           fields="ensembl,symbol", ensemblonly=False, return_full=False):
    """
    Takes a list of gene identifiers to map to a different type of identifier or multiple identifiers. For Ensembl IDs
    and symbols first check in a gtf-file, all not found there are queried via the API of https://mygene.info/
    (how to cite: https://mygene.info/citation/). See the documentation of mygene for further specifications
    of the options: https://docs.mygene.info/en/latest/doc/query_service.html. Note, there might be cases where
    multiple matches are found per gene. For Ensembl IDs the first one is taken. Pay attention to upper case and lower case of symbols.
    Name mapping is fun.

    Args:
        gene_identifiers: List of identifiers, e.g. symbols, Ensembl IDs or Entrez IDs.
        gtf_file: Gene annotation in gtf-style, can be gzipped. Only needed for symbols and Ensembl IDs.
        species: Give the name of the species, see below for the available ones.
        scopes: Where mygene.info will search for the gene symbols. For Ensembl ID use 'ensembl.gene'.
        fields: csv-string to which fields the output will be limited to. E.g. 'ensembl,symbol'. Can also be 'all'.
        ensemblonly: If only to return the hits with valid Ensembl gene ids.
        return_full: If to return the full dictionary as it is received by mygene. Contains more information such as
            protein or transcript ids which are in nested dictionaries from Ensembl. In this case, doesn't include
            any information that was found in the gtf file.

    Returns:
        tuple:
            - **mapped_identifiers**: Dict of {identifier: {field: field ID for each field}} of the mappable identifiers.
            - **no_hits**: Identifiers which were not mappable via gtf-file nor mygene.info.
    """
    gene_identifiers = [str(x) for x in gene_identifiers]  # In case IDs are given as IDs.
    available_species = ['human', 'mouse', 'rat', 'fruitfly', 'nematode', 'zebrafish', 'thale-cress', 'frog', 'pig']
    if species not in available_species:
        print("ERROR: species not available with name for mygene.info", species)
        print("Available are", available_species)
        return

    mapped_identifiers = {}
    if not return_full and gtf_file and ('ensembl' in fields or 'symbol' in fields):
        print("Checking gtf-file for matches")
        if gtf_file.endswith('.gz'):
            opener = gzip.open(gtf_file, 'rt')
        else:
            opener = open(gtf_file)
        for line in opener:
            if not line.startswith("#") and line.split('\t')[2] == 'gene':
                gene_id = line.split('\t')[8].split('gene_id "')[-1].split('";')[0].split('.')[0]
                gene_name = line.split('\t')[8].split('gene_name "')[-1].split('";')[0]
                if gene_id in gene_identifiers or gene_name in gene_identifiers:
                    mapped_identifiers[gene_name if gene_name in gene_identifiers else gene_id] = {'ensembl': gene_id,
                                                                                                   'symbol': gene_name}
        gtf_missed = [g for g in gene_identifiers if g not in mapped_identifiers]

    else:
        gtf_missed = gene_identifiers

    # Use the API from mygene for those we can't find in the gtf-file.
    if len(gtf_missed) > 0:
        print("Querying mygene for names", len(gtf_missed))
        query_data = {'species': species,
                      'scopes': scopes,
                      'fields': fields,
                      'ensemblonly': str(ensemblonly).lower()}
        query_n = len(gtf_missed)
        query_time = time.time()
        if query_n <= 1000:
            query_data['q'] = ' '.join(gtf_missed)
            res = requests.post('https://mygene.info/v3/query', query_data)
            res_json = res.json()
        else:
            # If the query is too long, we will need to break it up into chunks of 1000 query genes (MyGene.info cap)
            if query_n % 1000 == 0:
                chunks = query_n / 1000
            else:
                chunks = (query_n / 1000) + 1
            query_chunks = []
            for i in range(math.ceil(chunks)):
                start_i, end_i = i*1000, (i+1)*1000
                query_chunks.append(' '.join(gtf_missed[start_i:end_i]))
            res_json = []
            for chunk in query_chunks:
                query_data['q'] = chunk
                res = requests.post('https://mygene.info/v3/query', query_data)
                res_json = res_json+list(res.json())
        print('Batch query complete:', round(time.time()-query_time, 2), 'seconds')

        if return_full:
            return res_json

        for entry in res_json[:len(gtf_missed)]:  # There can be appended entries that are just plain strings.
            if ensemblonly:
                if 'ensembl' not in entry:
                    continue
            for field in fields.split(','):
                if field in entry:
                    if entry['query'] not in mapped_identifiers:
                        mapped_identifiers[entry['query']] = {}
                    if field == 'ensembl' or field == 'ensembl.gene':  # There it's nested.
                        if type(entry['ensembl']) == list:
                            mapped_identifiers[entry['query']][field] = entry['ensembl'][0]['gene']  # Only keep the first hit.
                        else:
                            mapped_identifiers[entry['query']][field] = entry['ensembl']['gene']
                    else:
                        mapped_identifiers[entry['query']][field] = entry[field]

    total_missed = [g for g in gene_identifiers if g not in mapped_identifiers]
    print("Non-matchable names", len(total_missed))
    return mapped_identifiers, total_missed



