#!/bin/bash
cat <<EOF
For more options and help: ./purge-chain.sh --help
Description: Purges local Substrate chains for DeepBrain Chain.
Authors: "DeepBrain Chain Community <community@dbc.team>", "Luke Penrod <luke@dbc.team>"
GitHub Organization: https://github.com/dbc-community

EOF

set -e

SCRIPT_VERSION="0.1.0"

# * Default messages catagories *
COLOR_DEFAULT="\033[0m"
COLOR_ERROR="\033[91m"
COLOR_INFO="\033[96m"
COLOR_SUCCESS="\033[92m"
COLOR_WARNING="\033[93m"

# * Configure directories *
PROJECT_SCRIPTS="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
cd "$PROJECT_SCRIPTS/.."
PROJECT_ROOT=$PWD

if [[ ! -f Cargo.toml ]]; then
	echo -e "${COLOR_ERROR}Cargo.toml not found, are you in the repository folder?\nCurrent directory: $PWD${COLOR_DEFAULT}"
	exit 1
fi

prompt_purge() {
	echo -e "Would you like to purge? ${COLOR_WARNING}(WARNING! this will delete everything)${COLOR_DEFAULT} [Yes/No]: "
	select yn in "Yes" "No"; do
		case $yn in
		Yes)
			echo -e "Purging #$dirCount: ${COLOR_INFO}$dirFullPath${COLOR_DEFAULT}"
			rm -rf $dirFullPath
			break
			;;
		No) break;;
		esac
	done
}

find_chain_name() {
	CHAIN_NAME_AUTO="$(find $PWD/target -maxdepth 2 -perm -111 -type f | sed 's#.*/##' | head -n 1)"
	if [[ -n $CHAIN_NAME_AUTO ]] && [[ -n CHAIN_NAME ]]; then
		CHAIN_NAME=$CHAIN_NAME_AUTO
		echo -e "${COLOR_DEFAULT}Found chain executable name: ${COLOR_INFO}${CHAIN_NAME}${COLOR_DEFAULT}"
	fi
}

print_help() {
	echo -e "Cleans local chain specs for DeepBrain Chain.\n" \
		"Usage ./purge-chain.sh <option>\n" \
		"\`$0 --verbose|-v\` \t\tset the bash script to verbose mode.  Great for troubleshooting.\n" \
		"\`$0 --chain-name NAME\` \tsets the name of the chain; i.e. dbc-mainchain\n" \
		"\`$0 -V\` \t\t\tdisplays the script version.\n"
}

purge_chain() {
	if [[ "$OSTYPE" == "linux-gnu" ]]; then
		OS_CHAIN_DIR="$HOME/.local/share/"
		echo -e "$OSTYPE detected!  Setting OS_CHAIN_DIR=$OS_CHAIN_DIR"
	elif [[ "$OSTYPE" == "darwin"* ]]; then
		echo -e "$OSTYPE detected!"
		OS_CHAIN_DIR="$HOME/Library/Application Support/"
	fi
	if [[ -z $CHAIN_NAME ]] || [[ ! -d $OS_CHAIN_DIR$CHAIN_NAME ]]; then
		echo -e "${COLOR_ERROR}[ERROR]: ${COLOR_WARNING}Could not find an executable for the chain!${COLOR_DEFAULT}" \
			"\n\t ${COLOR_INFO}Missing executable path:${COLOR_DEFAULT} $PWD/target/*" \
			"\n\n\t ${COLOR_INFO}Possible remedies:${COLOR_DEFAULT}\n\t ------------------" \
			"\n\t ${COLOR_INFO}Option 1)${COLOR_DEFAULT} cd ${PROJECT_ROOT};./scripts/init.sh;./scripts/build.sh;cargo build --release" \
			"\n\t ${COLOR_INFO}Option 2)${COLOR_DEFAULT} Run ./purge-chain.sh --chain-name CHAINNAME\n"
	fi
	if [[ -d "$OS_CHAIN_DIR$CHAIN_NAME/chains" ]]; then
		echo -e "Listing chains inside of: ${COLOR_INFO}$OS_CHAIN_DIR$CHAIN_NAME/chains${COLOR_DEFAULT}"
		cd $OS_CHAIN_DIR$CHAIN_NAME/chains
		dirCount=0
		for dir in */; do
			dirCount=$((dirCount + 1))
			if [[ -d "$dir" ]]; then
				dirFullPath=$PWD/$dir
				echo -e "Chain #$dirCount: ${COLOR_INFO}$dir${COLOR_DEFAULT}"
				prompt_purge
			fi
		done
	fi
	return 0
}

case $1 in
-h | --help)
	print_help
	exit 0
	;;
-v | --verbose)
	echo "Verbose mode set -x"
	set -x
	find_chain_name
	purge_chain
	;;
-V)
	echo -e "${COLOR_DEFAULT}Version: ${COLOR_INFO}${SCRIPT_VERSION}${COLOR_DEFAULT}"
	exit 0
	;;
--chain-name)
	CHAIN_NAME=$2
	echo -e "${COLOR_DEFAULT}Updated chain name to: ${COLOR_INFO}${CHAIN_NAME}${COLOR_DEFAULT}"
	purge_chain
	;;

'')
	find_chain_name
	purge_chain
	;;
*)
	echo "Invalid option"
	print_help
	exit 0
	;;
esac
