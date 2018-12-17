FROM phusion/baseimage:0.11 AS builder
LABEL maintainer="chevdor@gmail.com"
LABEL description="This is the build stage. Here we create the binary."

RUN install_clean build-essential \
    pkg-config \
    clang \
    libssl-dev \
    openssl \
    cmake \
    cargo

ARG PROFILE=release
WORKDIR /edgeware

COPY . /edgeware

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM phusion/baseimage:0.11
LABEL maintainer="chevdor@gmail.com"
LABEL description="This is the 2nd stage: a very small image where we copy the Edgeware binary."
ARG PROFILE=release
COPY --from=builder /edgeware/target/$PROFILE/edgeware /usr/local/bin

RUN rm -rf /usr/lib/python* && \
	mkdir -p /root/.local/share/Substrate && \
	ln -s /root/.local/share/Substrate /data

EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/edgeware", "--dev"]
