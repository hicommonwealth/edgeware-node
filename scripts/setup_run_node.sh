SEED=$(subkey generate | sed -n 2p | cut  -d":" -f2 | awk '{$1=$1};1' | cut -c 3-)

echo "#!/bin/bash" >> ~/edgeware-node/run_node.sh
echo "CHAINSPEC=\${1:-chains/testnet-1.0.0.json}" >> ~/edgeware-node/run_node.sh
echo "NODE_NAME=$HOSTNAME" >> ~/edgeware-node/run_node.sh
echo "" >> ~/edgeware-node/run_node.sh
echo "echo \"\$NODE_NAME running \$CHAINSPEC\"" >> ~/edgeware-node/run_node.sh
echo "" >> ~/edgeware-node/run_node.sh
echo "target/release/edgeware \\" >> ~/edgeware-node/run_node.sh
echo "    --node-key=$SEED \\" >> ~/edgeware-node/run_node.sh
echo "    --chain=\$CHAINSPEC \\" >> ~/edgeware-node/run_node.sh
echo "    --ws-external \\" >> ~/edgeware-node/run_node.sh
echo "    --rpc-cors="*" \\" >> ~/edgeware-node/run_node.sh
echo "    --validator \\" >> ~/edgeware-node/run_node.sh
echo "    --name \$NODE_NAME" >> ~/edgeware-node/run_node.sh

chmod +x ~/edgeware-node/run_node.sh