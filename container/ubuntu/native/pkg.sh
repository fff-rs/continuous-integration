#!/usr/bin/env bash
set -e

apt-get -y --no-install-recommends console-data

loadkeys us || echo "loadkeys failed {#?}"
setxkbmap us || echo "setxkbmap failed {#?}"

apt-get -y install curl wget git \
	capnproto \
	pkg-config \
	libopenblas-{dev,base}

dpkg -i /tmp/libcudnn7*.deb

