# use Debian 10 instead of phusion/baseimage:0.11 otherwise it installs Debian 4 and cannot install
# latest Node.js 12 (only Node.js 8)
FROM debian:buster AS builder
LABEL maintainer="jake@commonwealth.im"
LABEL description="This is the build stage. Here we create the binary."

ARG PROFILE=release

# DEFAULT SETUP
WORKDIR /edgeware

COPY . /edgeware
# RUN /edgeware/git checkout v0.5.0

RUN apt-get update && \
	apt-get install -y build-essential cmake pkg-config libssl-dev openssl git clang libclang-dev && \
	apt-get install -y curl vim unzip screen sudo && \
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

FROM debian:buster
LABEL maintainer="hello@commonwealth.im"
LABEL description="This is the 2nd stage: a very small image where we copy the Edgeware binary."
# https://github.com/phusion/baseimage-docker/issues/319
ENV DEBIAN_FRONTEND noninteractive
ARG PROFILE=release
COPY --from=builder /edgeware/target/$PROFILE/edgeware /usr/local/bin
COPY --from=builder /edgeware/mainnet /usr/local/bin/mainnet
COPY --from=builder /edgeware/testnets /usr/local/bin/testnets
# latest Node.js 12.x https://github.com/nodesource/distributions#installation-instructions
RUN rm -rf /usr/lib/python* && \
	mkdir -p /root/.local/share && \
	ln -s /root/.local/share /data \
	cd /usr/local/bin && \
	apt-get update && \
	apt-get install -y cmake && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends apt-utils && \
	apt-get install -y curl screen && \
	curl -sL https://deb.nodesource.com/setup_12.x | bash - && \
	apt-get install -y nodejs npm

EXPOSE 30333 30344 9933 9944
VOLUME ["/data"]

WORKDIR /usr/local/bin

RUN echo $PWD
ENV DEBIAN_FRONTEND teletype
