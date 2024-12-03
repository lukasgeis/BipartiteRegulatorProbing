cargo build --release --bin tf_networks
BINARY="./target/release/tf_networks"

OUTPUTDIR="data"
mkdir -p $OUTPUTDIR

LEUKEMIA="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Sub_TCellLeukemia.txt"
LEUKEMIA_SHUFFLED="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Shuffled_TFBS_Sub_TCellLeukemia.txt"
MEDIATED="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Sub_T_CELL_MEDIATED_IMMUNITY.txt"
MEDIATED_SHUFFLED="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Shuffled_TFBS_Sub_T_CELL_MEDIATED_IMMUNITY.txt"
RANDOM_GENES="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Sub_RandomGenes.txt"

echo "Leukemia"
$BINARY --file $LEUKEMIA -k 30 -i 10 >> "${OUTPUTDIR}/leukemia.json"
$BINARY --file $LEUKEMIA -k 40 -i 10 >> "${OUTPUTDIR}/leukemia.json"
$BINARY --file $LEUKEMIA -k 50 -i 10 >> "${OUTPUTDIR}/leukemia.json"

echo "Leukemia-Shuffled"
$BINARY --file $LEUKEMIA_SHUFFLED -k 30 -i 10 >> "${OUTPUTDIR}/leukemia_shuffled.json"
$BINARY --file $LEUKEMIA_SHUFFLED -k 40 -i 10 >> "${OUTPUTDIR}/leukemia_shuffled.json"
$BINARY --file $LEUKEMIA_SHUFFLED -k 50 -i 10 >> "${OUTPUTDIR}/leukemia_shuffled.json"

echo "Mediated"
$BINARY --file $MEDIATED -k 30 -i 10 >> "${OUTPUTDIR}/mediated.json"
$BINARY --file $MEDIATED -k 40 -i 10 >> "${OUTPUTDIR}/mediated.json"
$BINARY --file $MEDIATED -k 50 -i 10 >> "${OUTPUTDIR}/mediated.json"

echo "Mediated-Shuffled"
$BINARY --file $MEDIATED_SHUFFLED -k 30 -i 10 >> "${OUTPUTDIR}/mediated_shuffled.json"
$BINARY --file $MEDIATED_SHUFFLED -k 40 -i 10 >> "${OUTPUTDIR}/mediated_shuffled.json"
$BINARY --file $MEDIATED_SHUFFLED -k 50 -i 10 >> "${OUTPUTDIR}/mediated_shuffled.json"

echo "Random"
$BINARY --file $RANDOM_GENES -k 30 -i 10 >> "${OUTPUTDIR}/random_genes.json"
$BINARY --file $RANDOM_GENES -k 40 -i 10 >> "${OUTPUTDIR}/random_genes.json"
$BINARY --file $RANDOM_GENES -k 50 -i 10 >> "${OUTPUTDIR}/random_genes.json"

