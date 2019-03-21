curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env  
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
rustup update stable
cargo install --git https://github.com/alexcrichton/wasm-gc

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	echo "Found linux"
	sudo apt install cmake pkg-config libssl-dev git clang libclang-dev
elif [[ "$OSTYPE" == "darwin"* ]]; then
	echo "Found macbook"
	brew install cmake pkg-config openssl git llvm
fi

cargo build --release
