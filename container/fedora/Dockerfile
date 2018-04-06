FROM fedora:latest
MAINTAINER Bernhard Schuster "bernhard@spearow.io"

COPY pkg.sh /tmp/pkg.sh

RUN chmod +x /tmp/pkg.sh && \
	/tmp/pkg.sh && \
	dnf clean all

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

RUN cargo install fd-find
RUN cargo install exa

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make

COPY escalate.sh /usr/local/bin/escalate
COPY prepare.sh /usr/local/bin/prepare

RUN chmod +x /usr/local/bin/*
