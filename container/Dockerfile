FROM ubuntu:latest
MAINTAINER Bernhard Schuster "bernhard@ratpoison.io"

RUN apt-get update -y
RUN apt-get install -y cmake make curl

RUN curl https://sh.rustup.rs -sSf | \
	sh -s -- --default-toolchain nightly -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make
