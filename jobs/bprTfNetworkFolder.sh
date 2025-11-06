cargo build --release --bin tf_networks
BINARY="./target/release/tf_networks"

OUTPUTDIR="tf_data"
mkdir -p $OUTPUTDIR

FOLDER="tf_gene_networks/Revision-Networks"

ITERATIONS=500

for KVAL in 30 40 50
do
    for FILE in "$FOLDER"/*; do
        FILENAME=$(basename "$FILE")
        $BINARY --file "$FOLDER/$FILENAME" -k $KVAL -i $ITERATIONS >> "$OUTPUTDIR/$FILENAME.json" &
    done

    wait
    echo "Round ${KVAL} done"
done

