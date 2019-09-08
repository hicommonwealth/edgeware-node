mkdir -p ~/.edgeware
for i in 1 2 3 4 5 6 7 8 9 10; do
    ./scripts/keygen.sh > ~/.edgeware/$1-$i.txt
done
