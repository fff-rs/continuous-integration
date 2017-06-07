FROM fedora:latest
MAINTAINER Bernhard Schuster "bernhard@ratpoison.io"

RUN dnf update -y \
	&& dnf install -y cmake make curl openblas-devel dnf-plugins-core \
	&& dnf config-manager -y --add-repo=http://negativo17.org/repos/fedora-nvidia.repo \
	&& dnf install -y compat-gcc-53 cuda-cudnn5.1 cuda-cudnn5.1-devel cuda cuda-devel cuda-cublas cuda-cublas-devel cuda-cudart cuda-cudart-devel \
	&& dnf install -y clang \
	&& dnf install -y openssl-static openssl-libs openssl-devel \
	&& dnf install -y capnproto capnproto-libs capnproto-devel \
	&& dnf clean all \
	&& curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make
