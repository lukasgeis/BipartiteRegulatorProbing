PYFILE="python3 scripts/occ_tfs.py"

mkdir -p "data/occ"

for TYPE in "leukemia" "leukemia_shuffled" "mediated" "mediated_shuffled" "random_genes"
do
    $PYFILE "data/${TYPE}.json" > "data/occ/${TYPE}.txt" 
done
