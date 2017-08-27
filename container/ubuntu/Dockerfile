curl https://sh.rustup.rs -sSf | sh

FROM ubuntu:latest
MAINTAINER Bernhard Schuster "bernhard@spearow.io"

COPY pkg.sh /tmp/pkg.sh

RUN apt-get update -y
RUN chmod +x /tmp/pkg.sh && \
	/tmp/pkg.sh && \
	apt-get clean

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make

