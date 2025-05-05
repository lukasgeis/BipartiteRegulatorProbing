cargo build --release --bin tf_networks
BINARY="./target/release/tf_networks"

OUTPUTDIR="data/ext"
mkdir -p $OUTPUTDIR

LEUKEMIA="tf_networks_data/leukemia.txt"
MEDIATED="tf_networks_data/mediated.txt"
RANDOM_GENES="tf_networks_data/random_genes.txt"

ITERATIONS=10

for LVAL in 3 4 5 6 7 8 9 10
do
    for NUM in 1 2 3 4 5 6 7 8 9 10 
    do
        $BINARY --file $LEUKEMIA -k 50 -i $ITERATIONS -l $LVAL --noopt >> "${OUTPUTDIR}/leukemia_${LVAL}_${NUM}.json" &
        $BINARY --file $MEDIATED -k 50 -i $ITERATIONS -l $LVAL --noopt >> "${OUTPUTDIR}/mediated_${LVAL}_${NUM}.json" &
        $BINARY --file $RANDOM_GENES -k 50 -i $ITERATIONS -l $LVAL --noopt >> "${OUTPUTDIR}/random_genes_${LVAL}_${NUM}.json" &
    done
done

echo "Done!"

wait
