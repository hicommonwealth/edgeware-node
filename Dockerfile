FROM phusion/baseimage:0.11 AS builder
LABEL maintainer="jake@commonwealth.im"
LABEL description="This is the build stage. Here we create the binary."

ARG PROFILE=release

# DEFAULT SETUP
WORKDIR /edgeware

COPY . /edgeware
# RUN /edgeware/git checkout v0.5.0
RUN /edgeware/setup.sh

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
