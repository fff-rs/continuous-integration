#!/usr/bin/env bash
set -e
apt-get -y install curl wget git \
	capnproto \
	pkg-config \
	libopenblas-{dev,base}

dpkg -i /tmp/libcudnn7*.deb

