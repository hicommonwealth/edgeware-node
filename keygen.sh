default=$(./target/release/subkey -p edgeware generate | grep -o '`.*`' | tr -d '`')
mnemonic=${1:-$default}

stash=$(./target/release/subkey inspect "${mnemonic}//stash")
controller=$(./target/release/subkey inspect "${mnemonic}//controller")

base_pubkey=$(./target/release/subkey inspect "${mnemonic}//base" | grep -o ': .*' | sed '1!d' | tr -d ': ')
base_address=$(./target/release/subkey inspect "${mnemonic}//base" | grep -o ': .*' | sed '2!d' | tr -d ': ')

stash_pubkey=$(./target/release/subkey inspect "${mnemonic}//stash" | grep -o ': .*' | sed '1!d' | tr -d ': ')
stash_address=$(./target/release/subkey inspect "${mnemonic}//stash" | grep -o ': .*' | sed '2!d' | tr -d ': ')

controller_pubkey=$(./target/release/subkey inspect "${mnemonic}//controller" | grep -o ': .*' | sed '1!d' | tr -d ': ')
controller_address=$(./target/release/subkey inspect "${mnemonic}//controller" | grep -o ': .*' | sed '2!d' | tr -d ': ')


echo "Mnemonic: ${mnemonic}"
echo "Base pubkey: ${base_pubkey}"
echo "Base address: ${base_address}"
echo ""
echo "Stash pubkey: ${stash_pubkey}"
echo "Stash address: ${stash_address}"
echo ""
echo "Controller pubkey: ${controller_pubkey}"
echo "Controller address: ${controller_address}"