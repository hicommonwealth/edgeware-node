FROM phusion/baseimage:0.11 AS builder
LABEL maintainer="jake@commonwealth.im"
LABEL description="This is the build stage. Here we create the binary."

ARG PROFILE=release

# DEFAULT SETUP
WORKDIR /edgeware

COPY . /edgeware
# RUN /edgeware/git checkout v0.5.0

RUN apt-get update && \
	apt-get install -y build-essential cmake pkg-config libssl-dev openssl git clang libclang-dev && \
	apt-get install -y vim unzip screen sudo && \
	# wget -O - https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2019-06-30
	curl https://sh.rustup.rs -sSf | sh -s -- -y && \
	# source $HOME/.cargo/env && \
	# export PATH=$HOME/.cargo/bin:$PATH && \
	echo 'PATH="$/root/.cargo/bin:$PATH";' >> ~/.bash_profile && \
  	. ~/.bash_profile && . /root/.cargo/env && \
	# `rustup uninstall` if any Git issues
	rustup update stable && \
	rustup update nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly && \
	cargo --version && \
	cargo install --git https://github.com/alexcrichton/wasm-gc && \
	# `cargo +nightly-2019-06-30 build --release` to override the default
	# version of cargo/rust being used with a specific version, for
	# if version of rust nightly has issues.
	# or switch first with `rustup default nightly-2019-06-30`
	cargo build --release
	# Check that your Cargo.lock file is the same as upstream and also
	# rust is up to date each time
	# Do not run `cargo update`

# # TROUBLESHOOTING SETUP #1
# RUN wget https://github.com/hicommonwealth/edgeware-node/archive/master.zip && \
# 	unzip master.zip
# WORKDIR /edgeware
# COPY ./edgeware-node-master /edgeware
# RUN /edgeware/setup.sh

# # TROUBLESHOOTING SETUP #2
# RUN git clone https://github.com/hicommonwealth/edgeware-node.git
# WORKDIR /edgeware
# COPY ./edgeware-node /edgeware
# RUN /edgeware/setup.sh

# ./scripts/purge-chain.sh

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11
LABEL maintainer="hello@commonwealth.im"
LABEL description="This is the 2nd stage: a very small image where we copy the Edgeware binary."
ARG PROFILE=release
COPY --from=builder /edgeware/target/$PROFILE/edgeware /usr/local/bin
COPY --from=builder /edgeware/testnets /usr/local/bin/testnets

RUN rm -rf /usr/lib/python* && \
	mkdir -p /root/.local/share && \
	ln -s /root/.local/share /data

EXPOSE 30333 30344 9933 9944
VOLUME ["/data"]

WORKDIR /usr/local/bin

RUN echo $PWD
