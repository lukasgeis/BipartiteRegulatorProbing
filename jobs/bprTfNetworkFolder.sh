cargo build --release --bin tf_networks
BINARY="./target/release/tf_networks"

OUTPUTDIR="tf_data"
mkdir -p $OUTPUTDIR

FOLDER="tf_gene_networks/Revision-Networks"

ITERATIONS=500

for KVAL in 30 40 50
do
    for FILE in "$FOLDER"/*; do
        $BINARY --file $FILE -k $KVAL -i $ITERATIONS >> "$OUTPUTDIR/$FILE.json" &
    done

    wait
    echo "Round ${KVAL} done"
done

