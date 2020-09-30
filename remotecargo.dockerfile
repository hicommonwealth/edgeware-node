FROM phusion/baseimage:focal-1.0.0alpha1-amd64
LABEL maintainer="hello@commonwealth.im"
LABEL description="A very small image where we copy the Edgeware binary"

COPY . /edgeware

RUN rm -rf /usr/lib/python* && \
	mkdir -p /root/.local/share && \
	ln -s /root/.local/share /data

EXPOSE 30333 9933 9944
VOLUME ["/data"]

WORKDIR /edgeware
ENTRYPOINT [ "/edgeware/target/release/edgeware" ]
