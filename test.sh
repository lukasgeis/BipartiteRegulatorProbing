for file in ../Old/data/0/*
do
    BASENAME="$(basename -- $file)"
    echo /scratch/memhierarchy/geis/bpr/logs/poly/OUT_$BASENAME
done

