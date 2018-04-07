FROM quay.io/spearow/machine-learning-container-fedora-base:latest
MAINTAINER Bernhard Schuster "bernhard@spearow.io"

COPY fedora/default/pkg.sh /tmp/pkg.sh
COPY fedora/default/tooling.sh /tmp/tooling.sh

RUN chmod +x /tmp/*.sh && \
	/tmp/pkg.sh && \
	/tmp/tooling.sh && \
	dnf clean all

COPY escalate.sh /usr/local/bin/escalate
COPY fedora/default/prepare.sh /usr/local/bin/prepare

RUN chmod +x /usr/local/bin/*
