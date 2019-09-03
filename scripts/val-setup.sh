ts-node src/index.ts -r testnet2.edgewa.re:9944 -s "${mnemonic}//stash" staking bond $controller_b58address $bond $reward_dest
ts-node src/index.ts -r testnet2.edgewa.re:9944 -s "${mnemonic}//controller" session setKeys $pubkey1,$pubkey2,$pubkey3
ts-node src/index.ts -s "${mnemonic}"//controller staking validate 3 0
