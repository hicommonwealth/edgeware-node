#!/bin/bash
db=${1:-all}

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Clearing local data from home dir: $HOME/.local/share/edgeware"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/edgeware/chains/staging-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/db/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/edgeware/chains/dev/db/
		rm -rf ~/.local/share/edgeware/chains/development/db/
	elif [[ "$db" == "edgeware" ]]; then
		rm -rf ~/.local/share/edgeware/chains/edgeware/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware_testnet/db/
	else
		rm -rf ~/.local/share/edgeware/chains/local-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/local_testnet/db/
		rm -rf ~/.local/share/edgeware/chains/staging-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/db/
		rm -rf ~/.local/share/edgeware/chains/dev/db/
		rm -rf ~/.local/share/edgeware/chains/development/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware_testnet/db/
        rm -rf ~/.local/share/edgeware/chains/$db/db/
	fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Clearing local data from home dir: $HOME/Library/Application Support/edgeware"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/staging-testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/staging_testnet/db/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/dev/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/development/db/
	elif [[ "$db" == "edgeware" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware-testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware_testnet/db/
	else
		rm -rf ~/Library/Application\ Support/edgeware/chains/local-testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/local_testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/staging-testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/staging_testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/dev/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/development/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware-testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware_testnet/db/
        rm -rf ~/Library/Application\ Support/edgeware/chains/$db/db/
	fi
else
  echo "Clearing local data from home dir: $HOME/.local/share/edgeware"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/db/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/edgeware/chains/dev/db/
		rm -rf ~/.local/share/edgeware/chains/development/db/
	elif [[ "$db" == "edgeware" ]]; then
		rm -rf ~/.local/share/edgeware/chains/edgeware/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware_testnet/db/
	else
		rm -rf ~/.local/share/edgeware/chains/local-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/local_testnet/db/
		rm -rf ~/.local/share/edgeware/chains/staging-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/db/
		rm -rf ~/.local/share/edgeware/chains/dev/db/
		rm -rf ~/.local/share/edgeware/chains/development/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware_testnet/db/
        rm -rf ~/.local/share/edgeware/chains/$db/db/
	fi
fi

echo "Deleted $db databases"
