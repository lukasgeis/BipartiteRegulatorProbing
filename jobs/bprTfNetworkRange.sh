cargo build --release --bin tf_networks
BINARY="./target/release/tf_networks"

OUTPUTDIR="data/ext"
mkdir -p $OUTPUTDIR

LEUKEMIA="tf_networks_data/leukemia.txt"
LEUKEMIA_SHUFFLED="tf_networks_data/leukemia_shuffled.txt"
MEDIATED="tf_networks_data/mediated.txt"
MEDIATED_SHUFFLED="tf_networks_data/mediated_shuffled.txt"
RANDOM_GENES="tf_networks_data/random_genes.txt"

ITERATIONS=10

for LVAL in 3 4 
do
    $BINARY --file $LEUKEMIA -k 50 -i $ITERATIONS -l $LVAL --noopt >> "${OUTPUTDIR}/leukemia_${LVAL}.json" &
    $BINARY --file $LEUKEMIA_SHUFFLED -k 50 -i $ITERATIONS -l $LVAL --noopt >> "${OUTPUTDIR}/leukemia_shuffled_${LVAL}.json" &
    $BINARY --file $MEDIATED -k 50 -i $ITERATIONS -l $LVAL --noopt >> "${OUTPUTDIR}/mediated_${LVAL}.json" &
    $BINARY --file $MEDIATED_SHUFFLED -k 50 -i $ITERATIONS -l $LVAL --noopt >> "${OUTPUTDIR}/mediated_shuffled_${LVAL}.json" & 
    $BINARY --file $RANDOM_GENES -k 50 -i $ITERATIONS -l $LVAL --noopt >> "${OUTPUTDIR}/random_genes_${LVAL}.json" &
done

wait
