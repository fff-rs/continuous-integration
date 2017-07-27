FROM fedora:latest
MAINTAINER Bernhard Schuster "bernhard@spearow.io"

RUN dnf update -y
RUN dnf install -y curl clang make cmake git &&  dnf clean dbcache
RUN dnf install -y openblas-static openblas-openmp openblas-devel  && dnf clean dbcache
RUN dnf install -y openssl-static openssl-libs openssl-devel  && dnf clean dbcache
RUN dnf install -y capnproto capnproto-libs capnproto-devel  && dnf clean dbcache
RUN dnf install -y dnf-plugins-core \
	&& dnf config-manager -y --add-repo=http://negativo17.org/repos/fedora-nvidia.repo \
	&& dnf install -y compat-gcc-53 \
			cuda-cudnn \
			cuda-cudnn-devel \
			cuda \
			cuda-devel \
			cuda-cublas \
			cuda-cublas-devel \
			cuda-cudart \
			cuda-cudart-devel \
	&& dnf clean all

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make
