#!/bin/bash
db=$1

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Clearing local data from home dir: $HOME/.local/share/edgeware"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/edgeware/chains/dev/
		rm -rf ~/.local/share/edgeware/chains/development/
	elif [[ "$db" == "edgeware" ]]; then
    	rm -rf ~/.local/share/edgeware/chains/edgeware/
    	rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
	else
		db="all"
	    rm -rf ~/.local/share/edgeware/chains/dev/
	    rm -rf ~/.local/share/edgeware/chains/development/
	    rm -rf ~/.local/share/edgeware/chains/edgeware/
	    rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
	    rm -rf ~/.local/share/edgeware/chains/staging_testnet/
    	rm -rf ~/.local/share/edgeware/chains/local_testnet/
	fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Clearing local data from home dir: $HOME/Library/Application Support/edgeware"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/staging_testnet/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/dev/
		rm -rf ~/Library/Application\ Support/edgeware/chains/development/
	elif [[ "$db" == "edgeware" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware/
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware-testnet/
	else
		db="all"
		rm -rf ~/Library/Application\ Support/edgeware/chains/dev/
		rm -rf ~/Library/Application\ Support/edgeware/chains/development/
	    rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware/
	    rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware-testnet/
	    rm -rf ~/Library/Application\ Support/edgeware/chains/staging_testnet/
		rm -rf ~/Library/Application\ Support/edgeware/chains/local_testnet/
	fi
else
  echo "Clearing local data from home dir: $HOME/.local/share/edgeware"
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/edgeware/chains/staging_testnet/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/edgeware/chains/dev/
		rm -rf ~/.local/share/edgeware/chains/development/
	elif [[ "$db" == "edgeware" ]]; then
    	rm -rf ~/.local/share/edgeware/chains/edgeware/
    	rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
	else
		db="all"
	    rm -rf ~/.local/share/edgeware/chains/dev/
	    rm -rf ~/.local/share/edgeware/chains/development/
	    rm -rf ~/.local/share/edgeware/chains/edgeware/
	    rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
	    rm -rf ~/.local/share/edgeware/chains/staging_testnet/
    	rm -rf ~/.local/share/edgeware/chains/local_testnet/
	fi
fi

echo "Deleted $db databases"
