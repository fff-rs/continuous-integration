#!/usr/bin/env bash
set -e
dnf install -y dnf-plugins-core \
	&& dnf config-manager -y --add-repo=https://negativo17.org/repos/fedora-nvidia.repo \
	&& dnf install -y curl clang make cmake git \
			openblas-static openblas-openmp openblas-devel \
			openssl-libs openssl-devel \
			capnproto capnproto-libs capnproto-devel \
			cuda-cudnn \
			cuda-cudnn-devel \
			cuda \
			cuda-devel \
			libcublas \
			libcublas-devel \
			cuda-cudart \
			cuda-cudart-devel \
			clinfo \
			ocl-icd \
			ocl-icd-devel \
			nvidia-driver-cuda \
			nvidia-driver-cuda \
	&& dnf erase -y beignet pocl
