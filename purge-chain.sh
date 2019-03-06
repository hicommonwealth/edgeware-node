db=$1

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/edgeware/chains/staging/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/edgeware/chains/development/
	elif [[ "$db" == "edgeware" ]]; then
    	rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
	else
		db="all"
	    rm -rf ~/.local/share/edgeware/chains/development/
	    rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
	    rm -rf ~/.local/share/edgeware/chains/staging/
	fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/staging/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/development/
	elif [[ "$db" == "edgeware" ]]; then
		rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware-testnet/
	else
		db="all"
		rm -rf ~/Library/Application\ Support/edgeware/chains/development/
	    rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware-testnet/
	    rm -rf ~/Library/Application\ Support/edgeware/chains/staging/
	fi
else
	if [[ "$db" == "staging" ]]; then
		rm -rf ~/.local/share/edgeware/chains/staging/
	elif [[ "$db" == "dev" ]]; then
		rm -rf ~/.local/share/edgeware/chains/development/
	elif [[ "$db" == "edgeware" ]]; then
    	rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
	else
		db="all"
	    rm -rf ~/.local/share/edgeware/chains/development/
	    rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
	    rm -rf ~/.local/share/edgeware/chains/staging/
	fi
fi

echo "Deleted $db databases"
