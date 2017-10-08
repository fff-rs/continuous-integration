#!/usr/bin/env bash
set -e
apt-get -y install \
	ca-certificates \
	curl wget git \
	capnproto \
	pkg-config \
	libopenblas-{dev,base}
