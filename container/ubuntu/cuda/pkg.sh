#!/usr/bin/env bash
set -e

apt-get -y --no-install-recommends console-data

loadkeys us || echo "loadkeys failed {#?}"
setxkbmap us || echo "setxkbmap failed {#?}"

apt-get -y --no-install-recommends install \
	curl wget git \
	capnproto \
	pkg-config \
	libopenblas-{dev,base} \
	cuda-cublas-dev-8-0 \
	cuda-minimal-build-8-0 \
	libcuda1-375 \
	nvidia-375-dev

dpkg -i /tmp/libcudnn7*.deb

