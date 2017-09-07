#!/usr/bin/env bash
set -e
apt-get -y install curl wget git \
	capnproto \
	pkg-config \
	nvidia-opencl-icd-375 \
	nvidia-libopencl1-375 \
	nvidia-375-dev
