FROM subversiveowl/pureos8-base:latest
MAINTAINER subversive-owl "subversive.owl@yandex.com"


RUN apt-get update -y

COPY pureos/cuda/pkg.sh /tmp/pkg.sh

RUN chmod +x /tmp/pkg.sh && \
	/tmp/pkg.sh && \
	apt-get clean

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make

COPY escalate.sh /usr/local/bin/escalate

RUN chmod +x /usr/local/bin/escalate
