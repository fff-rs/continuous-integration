FROM fedora:latest
LABEL maintainer="Bernhard Schuster <bernhard@spearow.io>"

RUN dnf update -y && \
	dnf install -y \
		"https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm" \
		"https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm" && \
	dnf install -y curl clang make cmake git \
		llvm-devel \
		valgrind \
		openssl-static openssl-libs openssl-devel \
		capnproto capnproto-libs capnproto-devel \
		micro \
		exa ripgrep hyperfine fd-find \
		dnf-plugins-core && \
	dnf clean all

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH

RUN cargo --version && rustc --version

RUN git config --global core.editor micro

ENV CMAKE_MAKE_PROGRAM=make
