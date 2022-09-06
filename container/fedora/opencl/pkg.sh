#!/usr/bin/env bash
set -e
dnf install -y dnf-plugins-core \
	&& dnf config-manager -y --add-repo=https://negativo17.org/repos/fedora-nvidia.repo \
	&& dnf install -y curl clang make cmake git \
			openblas-static openblas-openmp openblas-devel \
			openssl-libs openssl-devel \
			capnproto capnproto-libs capnproto-devel \
			clinfo \
			ocl-icd \
			ocl-icd-devel \
			nvidia-driver-cuda \
	&& dnf erase -y beignet pocl
# ocl-icd \ # OpenCL.so.1
# ocl-icd-devel \ # OpenCL.so
# nvidia-driver-cuda \ # contains nvidia.icd
# pocl # erase since this provides duplicate functionality which will cause errors
# beignet # installed as weak dependency of ocl-icd
