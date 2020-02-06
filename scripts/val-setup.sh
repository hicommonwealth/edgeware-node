$path_to_cli/edge -r testnet3.edgewa.re:9944 -s "${mnemonic}//stash" staking bond $controller_b58address $bond $reward_dest
$path_to_cli/edge -r testnet3.edgewa.re:9944 -s "${mnemonic}//controller" session setKeys $pubkey1,$pubkey2,$pubkey3,$pubkey4
$path_to_cli/edge -s "${mnemonic}"//controller staking validate 3 0
