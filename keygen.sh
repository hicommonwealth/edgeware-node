if [[ -z $(subkey) ]]; then
	cargo install --force --git https://github.com/paritytech/substrate subkey
fi

new_mnemonic=$(subkey generate | grep -o '`.*`' | tr -d '`')
mnemonic=${1:-$new_mnemonic}

stash_seed=$(subkey -e inspect "${mnemonic}"//stash | grep -o ': .*' | sed '1!d' | tr -d ': ')
stash_pubkey=$(subkey -e inspect "${mnemonic}"//stash | grep -o ': .*' | sed '2!d' | tr -d ': ')
stash_address=$(subkey -e inspect "${mnemonic}"//stash | grep -o ': .*' | sed '3!d' | tr -d ': ')

controller_seed=$(subkey -e inspect "${mnemonic}"//controller | grep -o ': .*' | sed '1!d' | tr -d ': ')
controller_pubkey=$(subkey -e inspect "${mnemonic}"//controller | grep -o ': .*' | sed '2!d' | tr -d ': ')
controller_address=$(subkey -e inspect "${mnemonic}"//controller | grep -o ': .*' | sed '3!d' | tr -d ': ')

authority_seed=$(subkey -e inspect "${mnemonic}"//authority | grep -o ': .*' | sed '1!d' | tr -d ': ')
authority_pubkey=$(subkey -e inspect "${mnemonic}"//authority | grep -o ': .*' | sed '2!d' | tr -d ': ')
authority_address=$(subkey -e inspect "${mnemonic}"//authority | grep -o ': .*' | sed '3!d' | tr -d ': ')

echo ""
echo "*********** SAVE THIS MNEMONIC FOR FUTURE USE OR RISK LOSING ACCESS TO ANY FUNDS ***********"
echo ""
echo "Mnemonic: ${mnemonic}"
echo ""
echo "********************************************************************************************"
echo ""
echo "STASH ACCOUNT FOR STORING FUNDS TO DELEGATE TO VALIDATORS OR GENERAL USE"
echo ""
echo "Stash seed: ${stash_seed}"
echo "Stash pubkey: ${stash_pubkey}"
echo "Stash address: ${stash_address}"
echo ""
echo "*********** CONTROLLER ACCOUNT FOR CONTROLLING A VALIDATOR NODE OR GENERAL USE ***********"
echo ""
echo "Controller seed: ${controller_seed}"
echo "Controller pubkey: ${controller_pubkey}"
echo "Controller address: ${controller_address}"
echo ""
echo "*********** AUTHORITY ACCOUNT FOR CONTROLLING A GRANDPA AUTHORITY/VALIDATOR NODE OR GENERAL USE ***********"
echo ""
echo "Authority seed: ${authority_seed}"
echo "Authority pubkey: ${authority_pubkey}"
echo "Authority address: ${authority_address}"
