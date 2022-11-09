FROM ubuntu:bionic

WORKDIR /opt/ollvm
RUN apt-get -y update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y   \
        build-essential \
        cmake \
        git \
        python3 && \
    git clone -b llvm-4.0 https://github.com/obfuscator-llvm/obfuscator.git /opt/obfuscator && \
    cmake -DCMAKE_BUILD_TYPE=Release -DLLVM_INCLUDE_TESTS=OFF /opt/obfuscator/ && \
    make -j7 && \
    rm -rf /opt/obfuscator && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    echo '#! /bin/sh\n/opt/ollvm/bin/clang $@' >> /bin/obfuscate && \
    chmod a+x /bin/obfuscate
