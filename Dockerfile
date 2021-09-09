#FROM paritytech/ci-linux:production as builder
FROM decentration/edgeware:v3.3.3 as builder

LABEL description="This is the build stage for edgeware. Here we create the binary."

ARG PROFILE=release
WORKDIR /edgeware

COPY . /edgeware/
#RUN  fallocate -l 1G /swapfile
RUN rustup uninstall nightly
RUN rustup install nightly-2021-05-31
RUN rustup update nightly
RUN rustup target add wasm32-unknown-unknown --toolchain nightly-2021-05-31


RUN cargo build --$PROFILE -j 1

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the edgeware binary."
ARG PROFILE=release
COPY --from=builder /edgeware/target/$PROFILE/edgeware /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /edgeware edgeware && \
	mkdir -p /edgeware/.local/share && \
	mkdir /data && \
	chown -R edgeware:edgeware /data && \
	ln -s /data /edgeware/.local/share/edgeware && \
	rm -rf /usr/bin /usr/sbin

USER edgeware
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/edgeware"]
    

