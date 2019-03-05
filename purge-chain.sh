if [[ "$OSTYPE" == "linux-gnu" ]]; then
	rm -rf ~/.local/share/edgeware/chains/development/
	rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
elif [[ "$OSTYPE" == "darwin"* ]]; then
    rm -rf ~/Library/Application\ Support/edgeware/chains/development/
    rm -rf ~/Library/Application\ Support/edgeware/chains/edgeware-testnet/
else
    rm -rf ~/.local/share/edgeware/chains/development/
    rm -rf ~/.local/share/edgeware/chains/edgeware-testnet/
fi
