#!/usr/bin/env bash
set -e

apt-get -y --no-install-recommends install \
	ca-certificates \
	curl wget git \
	capnproto \
	pkg-config \
	libopenblas-{dev,base} \
	cuda-cublas-dev-8-0 \
	cuda-minimal-build-8-0 \
	libcuda1-375 \
	nvidia-375-dev \
	libssl-dev

dpkg -i /tmp/libcudnn7*.deb

