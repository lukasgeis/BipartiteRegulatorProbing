cargo build --release --bin tf_networks
BINARY="./target/release/tf_networks"

OUTPUTDIR="data"
mkdir -p $OUTPUTDIR

LEUKEMIA="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Sub_TCellLeukemia.txt"
LEUKEMIA_SHUFFLED="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Shuffled_TFBS_Sub_TCellLeukemia.txt"
MEDIATED="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Sub_T_CELL_MEDIATED_IMMUNITY.txt"
MEDIATED_SHUFFLED="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Shuffled_TFBS_Sub_T_CELL_MEDIATED_IMMUNITY.txt"
RANDOM_GENES="tf_networks_data/IHECRE00000187_GeneTFMatrix_FracBoundCREs_Sub_RandomGenes.txt"

for KVAL in 30 40 50
do
    $BINARY --file $LEUKEMIA -k $KVAL -i 10 >> "${OUTPUTDIR}/leukemia.json" &
    $BINARY --file $LEUKEMIA_SHUFFLED -k $KVAL -i 10 >> "${OUTPUTDIR}/leukemia_shuffled.json" &
    $BINARY --file $MEDIATED -k $KVAL -i 10 >> "${OUTPUTDIR}/mediated.json" &
    $BINARY --file $MEDIATED_SHUFFLED -k $KVAL -i 10 >> "${OUTPUTDIR}/mediated_shuffled.json" & 
    $BINARY --file $RANDOM_GENES -k $KVAL -i 10 >> "${OUTPUTDIR}/random_genes.json" &

    wait
done

