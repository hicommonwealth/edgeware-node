#!/bin/bash
db=$1

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Clearing local data from home dir: $HOME/.local/share/edgeware"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/db/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/edgeware/chains/dev/db/
		rm -rf ~/.local/share/edgeware/chains/development/db/
	elif [[ "$db" == "edgeware" ]]; then
		rm -rf ~/.local/share/edgeware/chains/edgeware/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware_testnet/db/
	else
		db="all"
		rm -rf ~/.local/share/edgeware/chains/dev/db/
		rm -rf ~/.local/share/edgeware/chains/development/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware_testnet/db/
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/db/
		rm -rf ~/.local/share/edgeware/chains/local_testnet/db/
	fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Clearing local data from home dir: $HOME/Library/Application Support/edgeware"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/staging_testnet/db/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/dev/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/development/db/
	elif [[ "$db" == "edgeware" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware_testnet/db/
	else
		db="all"
		rm -rf ~/Library/Application\ Support/edgeware/chains/dev/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/development/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware_testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/staging_testnet/db/
		rm -rf ~/Library/Application\ Support/edgeware/chains/local_testnet/db/
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
		rm -rf ~/.local/share/edgeware/chains/edgeware_testnet/db/
	else
		db="all"
		rm -rf ~/.local/share/edgeware/chains/dev/db/
		rm -rf ~/.local/share/edgeware/chains/development/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware/db/
		rm -rf ~/.local/share/edgeware/chains/edgeware_testnet/db/
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/db/
		rm -rf ~/.local/share/edgeware/chains/local_testnet/db/
	fi
fi

echo "Deleted $db databases"
