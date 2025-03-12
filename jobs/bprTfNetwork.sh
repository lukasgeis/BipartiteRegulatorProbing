cargo build --release --bin tf_networks
BINARY="./target/release/tf_networks"

OUTPUTDIR="data"
mkdir -p $OUTPUTDIR

LEUKEMIA="tf_networks_data/leukemia.txt"
LEUKEMIA_SHUFFLED="tf_networks_data/leukemia_shuffled.txt"
MEDIATED="tf_networks_data/mediated.txt"
MEDIATED_SHUFFLED="tf_networks_data/mediated_shuffled.txt"
RANDOM_GENES="tf_networks_data/random_genes.txt"

ITERATIONS=500

for KVAL in 30 40 50
do
    $BINARY --file $LEUKEMIA -k $KVAL -i $ITERATIONS >> "${OUTPUTDIR}/leukemia.json" &
    $BINARY --file $LEUKEMIA_SHUFFLED -k $KVAL -i $ITERATIONS >> "${OUTPUTDIR}/leukemia_shuffled.json" &
    $BINARY --file $MEDIATED -k $KVAL -i $ITERATIONS >> "${OUTPUTDIR}/mediated.json" &
    $BINARY --file $MEDIATED_SHUFFLED -k $KVAL -i $ITERATIONS >> "${OUTPUTDIR}/mediated_shuffled.json" & 
    $BINARY --file $RANDOM_GENES -k $KVAL -i $ITERATIONS >> "${OUTPUTDIR}/random_genes.json" &

    wait
    echo "Round ${KVAL} done"
done

