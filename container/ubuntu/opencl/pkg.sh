#!/usr/bin/env bash
set -e
apt-get -y --no-install-recommends install \
	ca-certificates \
	curl wget git \
	capnproto \
	pkg-config \
	libopenblas-{dev,base} \
	nvidia-opencl-icd-375 \
	ocl-icd-opencl-dev \
	ocl-icd-libopencl1 \
	nvidia-375-dev \
	libssl-dev
