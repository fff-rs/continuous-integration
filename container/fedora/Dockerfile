FROM fedora:latest
MAINTAINER Bernhard Schuster "bernhard@spearow.io"

COPY escalate.sh /tmp/escalate.sh
COPY pkg.sh /tmp/pkg.sh

RUN dnf update -y
RUN chmod +x /tmp/pkg.sh && \
	/tmp/pkg.sh && \
	dnf clean all

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make


RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make

RUN chmod +x /tmp/escalate.sh && /tmp/escalate.sh
