FROM debian:buster

RUN useradd -m user
ENV HOME /home/user

ENV PATH /opt/tigress/3.1:$PATH
ENV TIGRESS_HOME /opt/tigress/3.1
COPY --chown=user:user resource /opt/

RUN apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
        gcc g++ perl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    chmod a+w /opt/tigress/3.1/* && \
    echo '#! /bin/sh\n/opt/tigress/3.1/tigress $@' >> /bin/obfuscate && \
    chmod a+x /bin/obfuscate

