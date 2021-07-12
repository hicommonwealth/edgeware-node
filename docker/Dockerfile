FROM ubuntu as builder

LABEL maintainer="jake@commonwealth.im"

ARG RUST_VERSION=nightly-2021-06-01
ARG PROFILE=release
ARG GIT_REPO=https://github.com/hicommonwealth/edgeware-node
ARG GIT_BRANCH=v3.3.3
ARG BINARY_NAME=edgeware
ARG PACKAGE_NAME=edgeware-cli

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    clang \
    cmake \
    curl \
    git \
    libssl-dev \
    pkg-config

# rustup is down for now -- using the raw github page instead
RUN curl https://raw.githubusercontent.com/rust-lang/rustup/master/rustup-init.sh -sSf | sh -s -- -y
# ENV PATH=$PATH:$HOME/.cargo/bin
RUN git clone ${GIT_REPO} source && cd source && git checkout ${GIT_BRANCH}
RUN $HOME/.cargo/bin/rustup default stable
RUN $HOME/.cargo/bin/rustup uninstall nightly 
RUN $HOME/.cargo/bin/rustup toolchain install ${RUST_VERSION} 
RUN $HOME/.cargo/bin/rustup target add wasm32-unknown-unknown --toolchain ${RUST_VERSION}
RUN cd source && $HOME/.cargo/bin/cargo build -p ${PACKAGE_NAME} --${PROFILE}
RUN cp source/target/${PROFILE}/${BINARY_NAME} /node
RUN cp -r source/chains /chains

# ===== SECOND STAGE ======

FROM ubuntu

COPY --from=builder /node /data/node
COPY --from=builder /chains /data/chains

# install curl in the event we want to interact with the local rpc
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y curl
RUN useradd --create-home runner

USER runner

EXPOSE 30333 9933 9944

ENTRYPOINT ["/data/node"]
