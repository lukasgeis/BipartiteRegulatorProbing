# Building TF-Gene networks

The algorithms shown on the main page of the repository can be applied to graphs made from transcription factors (TFs) and genes.
As described in the manuscript, we estimate the interaction between a TF and a gene by the number of a gene's regulatory elements (REs)
that have a binding site for the TF. Additionally, the number of REs of a gene with a binding site must be higher than expected, otherwise
the edge weight is set to zero. 
Here, we provide scripts to generate TF-gene networks also from custom data. The regions in which the TFs can potentially bind
need to be provided, as well as an interaction file, that tells which regions map to which gene.

The networks used in the manuscript can be found in [Tcell_networks](https://github.com/lukasgeis/BipartiteRegulatorProbing/tree/main/tf_gene_networks/Tcell_networks) 

## Installation
For the required packages, see the [requirements file](https://github.com/lukasgeis/BipartiteRegulatorProbing/blob/main/tf_gene_networks/requirements.txt). You can install them via
```bash
pip3 install -r scripts/requirements
```

The prediction of TF binding sites is done with [FIMO](https://meme-suite.org/meme/doc/fimo.html), so this needs to be installed as well.
Alternatively, you can also create a conda/mamba environment from the provided yaml-file, which installs FIMO as well:
```bash
conda env create -f condaenv.yml
conda activate tf_gene_networks
```


## Usage
The script to generate the network is called TFGeneNetwork.py, and it takes a JSON-styled file that defines all of the following options:

Parameter | Description 
--- |----------------------
`output_path` | Path to a folder that will be created and the output written into.  
`fimo_src` |  Path to the FIMO executable. If FIMO is on the PATH, you can just put "fimo". 
`gtf_file` | Gtf-file gene annotation, e.g. from [GENCODE](https://www.gencodegenes.org/).
`fasta_file` | Fasta file with the genome sequence. Also available on GENCODE. There also needs to be an index file for that fasta_file under `fasta_file+.fai` .
`pwm_file` | Path to the file with TF motifs in meme-format, e.g. from [JASPAR](https://jaspar.elixir.no/).
`open_tfs_only` | Boolean whether only TFs that have a peak from `peak_file` overlapping any of their promoters (± 200bp around any TSS) should be considered.
`peak_file` | Path to the bed-file with the regulatory regions.
`abc_interactions` | Path to the interactions between regulatory regions and genes. Expects the format as given by [STARE](https://github.com/schulzlab/stare). Custom files are also possible, if they have a header and entries for 'Ensembl ID' and 'PeakID', where the 'PeakID' is in the format like '21:1-100'.   
`cores` | How many CPU cores should be used for the parallelized part of the computation.

We provide a toy example that you can run:
```bash
python3 TFGeneNetworks.py ExampleRun_JSON.txt
```
From the example you can see how a JSON file should look like. The expected output of the toy run is in [ExampleRun/](https://github.com/lukasgeis/BipartiteRegulatorProbing/tree/main/tf_gene_networks/ExampleRun).
The input files used for the toy run are in [ExampleData/](https://github.com/lukasgeis/BipartiteRegulatorProbing/tree/main/tf_gene_networks/ExampleData).

A file called 'TFGeneNet.txt.gz' will be generated, that is a matrix of Genes*TFs filled with the edge weight. There is an additional first columns called '#CREs' that holds the number of regions interacting with the gene.
To get the network for your data, just provide a JSON file with your paths and options. The network will be generated for 
all genes that have interactions in `abc_interactions`. The network can then be subsetted for specific gene sets.

