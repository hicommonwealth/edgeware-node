export EDG_DIR=${EDG_DIR:='.'}
$EDG_DIR/purge-chain.sh all

if [ -f "$EDG_DIR/target/debug/edgeware" ]
then
    $EDG_DIR/target/debug/edgeware --chain=cwci --alice --validator --force-authoring
elif [ -f "$EDG_DIR/target/release/edgeware" ]
then
    $EDG_DIR/target/release/edgeware --chain=cwci --alice --validator --force-authoring
else
    echo "edgeware not found"
fi
