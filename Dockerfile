FROM alpine:edge AS builder
LABEL maintainer="chevdor@gmail.com"
LABEL description="This is the build stage for Substrate. Here we create the binary."

RUN apk add build-base \
    cmake \
    linux-headers \
    openssl-dev \
    clang \
    cargo

ARG PROFILE=release
WORKDIR /edgeware-node

COPY . /edgeware-node

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM alpine:edge
LABEL maintainer="chevdor@gmail.com"
LABEL description="This is the 2nd stage: a very small image where we copy the Edgeware binary."
ARG PROFILE=release
COPY --from=builder /edgeware-node/target/$PROFILE/edgeware /usr/local/bin

RUN apk add --no-cache ca-certificates \
    libstdc++ \
    openssl

RUN rm -rf /usr/lib/python* && \
	mkdir -p /root/.local/share/Substrate && \
	ln -s /root/.local/share/Substrate /data

EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/edgeware", "--dev"]
