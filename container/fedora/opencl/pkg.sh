#!/usr/bin/env bash
set -e
dnf install -y dnf-plugins-core \
	&& dnf config-manager -y --add-repo=http://negativo17.org/repos/fedora-nvidia.repo \
	&& dnf install -y curl clang make cmake git \
			openblas-static openblas-openmp openblas-devel \
			openssl-static openssl-libs openssl-devel \
			capnproto capnproto-libs capnproto-devel \
			compat-gcc-53 \
			clinfo \
			ocl-icd \
			ocl-icd-devel \
			nvidia-driver-cuda \
	&& dnf erase -y beignet
# ocl-icd \ # OpenCL.so.1
# ocl-icd-devel \ # OpenCL.so
# nvidia-driver-cuda \ # contains nvidia.icd
# beignet # installed as weak dependency of ocl-icd
