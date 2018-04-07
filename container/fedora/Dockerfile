FROM fedora:latest
MAINTAINER Bernhard Schuster "bernhard@spearow.io"

RUN dnf update -y && \
	dnf install -y \
		"https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm" \
		"https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm" && \
	dnf install -y curl clang make cmake git \
		openssl-static openssl-libs openssl-devel \
		capnproto capnproto-libs capnproto-devel \
		micro \
		exa ripgrep \
		dnf-plugins-core && \
	dnf copr -y enable drahnr/zshxt && \
	dnf install -y zshxt && \
	dnf clean all

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

RUN cargo install fd-find

RUN git config --global core.editor micro

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make