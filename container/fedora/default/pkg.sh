#!/usr/bin/env bash
set -e
dnf install -y dnf-plugins-core \
	&& dnf config-manager -y --add-repo=https://negativo17.org/repos/fedora-nvidia.repo \
	&& dnf install -y curl clang make cmake git \
			openblas-static openblas-openmp openblas-devel \
			openssl-static openssl-libs openssl-devel \
			capnproto capnproto-libs capnproto-devel \
			cuda-cudnn \
			cuda-cudnn-devel \
			cuda \
			cuda-devel \
			cuda-cublas \
			cuda-cublas-devel \
			cuda-cudart \
			cuda-cudart-devel \
			clinfo \
			ocl-icd \
			ocl-icd-devel \
			nvidia-driver-cuda-libs \
			nvidia-driver-cuda \
	&& dnf erase -y beignet pocl
# ocl-icd \ # OpenCL.so.1
# ocl-icd-devel \ # OpenCL.so
# nvidia-driver-cuda \ # contains nvidia.icd
# beignet # installed as weak dependency of ocl-icd
# pocl # erase since this provides duplicate functionality which will cause errors
# nvidia-driver-cuda provides nvidia-smi, which is helpful for debugging

