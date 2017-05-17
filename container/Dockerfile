FROM fedora:latest
MAINTAINER Bernhard Schuster "bernhard@ratpoison.io"

RUN dnf update -y
RUN dnf install -y cmake make curl openblas-devel dnf-plugins-core

RUN dnf config-manager -y --add-repo=http://negativo17.org/repos/fedora-nvidia.repo

RUN dnf install -y compat-gcc-53 cuda-cudnn5.1 cuda-cudnn5.1-devel cuda cuda-devel cuda-cublas cuda-cublas-devel cuda-cudart cuda-cudart-devel

RUN dnf install clang

RUN curl https://sh.rustup.rs -sSf | \
	sh -s -- --default-toolchain nightly -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make
