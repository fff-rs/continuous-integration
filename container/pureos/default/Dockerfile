FROM subversiveowl/pureos8-base:latest
MAINTAINER subversive-owl "subversive.owl@yandex.com"

COPY escalate.sh /usr/local/bin/escalate
COPY pkg.sh /tmp/pkg.sh

RUN apt-get update -y
RUN chmod +x /tmp/pkg.sh && \
	/tmp/pkg.sh && \
	apt-get clean

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y

ENV PATH=/root/.cargo/bin:$PATH
ENV CMAKE_MAKE_PROGRAM=make

RUN chmod +x /usr/local/bin/escalate
