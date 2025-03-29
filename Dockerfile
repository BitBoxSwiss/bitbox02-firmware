# Copyright 2019 Shift Cryptosecurity AG
# Copyright 2020 Shift Crypto AG
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# If you are building for a foreign target and you get segfaults, try the latest version of qemu
# $ docker pull tonistiigi/binfmt:latest
# $ docker run --privileged --rm tonistiigi/binfmt --uninstall qemu-*
# $ docker run --privileged --rm tonistiigi/binfmt --install arm64

FROM ubuntu:22.04
ENV DEBIAN_FRONTEND noninteractive

# These are automatically provided by docker (no need for --build-arg)
ARG TARGETPLATFORM
ARG TARGETARCH

RUN apt-get update && apt-get upgrade -y && apt-get install -y wget nano rsync curl gnupg2 jq unzip bzip2 xz-utils

# for clang-*-15, see https://apt.llvm.org/
RUN echo "deb http://apt.llvm.org/jammy/ llvm-toolchain-jammy-18 main" >> /etc/apt/sources.list && \
    echo "deb-src http://apt.llvm.org/jammy/ llvm-toolchain-jammy-18 main" >> /etc/apt/sources.list && \
    wget -O - https://apt.llvm.org/llvm-snapshot.gpg.key | apt-key add -

# Install gcc8-arm-none-eabi
RUN if [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
      GNU_TOOLCHAIN=https://developer.arm.com/-/media/Files/downloads/gnu/13.3.rel1/binrel/arm-gnu-toolchain-13.3.rel1-aarch64-arm-none-eabi.tar.xz \
      GNU_TOOLCHAIN_HASH=c8824bffd057afce2259f7618254e840715f33523a3d4e4294f471208f976764 \
      GNU_TOOLCHAIN_FORMAT=xz; \
    else \
      GNU_TOOLCHAIN=https://developer.arm.com/-/media/Files/downloads/gnu-rm/8-2018q4/gcc-arm-none-eabi-8-2018-q4-major-linux.tar.bz2 \
      GNU_TOOLCHAIN_HASH=fb31fbdfe08406ece43eef5df623c0b2deb8b53e405e2c878300f7a1f303ee52 \
      GNU_TOOLCHAIN_FORMAT=bz2; \
    fi; \
    wget -O gcc.tar.${GNU_TOOLCHAIN_FORMAT} ${GNU_TOOLCHAIN} &&\
    echo "$GNU_TOOLCHAIN_HASH gcc.tar.${GNU_TOOLCHAIN_FORMAT}" | sha256sum -c &&\
    tar -xvf gcc.tar.${GNU_TOOLCHAIN_FORMAT} -C /usr/local --strip-components=1 &&\
    rm -f gcc.tar.${GNU_TOOLCHAIN_FORMAT}

# Tools for building
RUN apt-get update && apt-get install -y \
    make \
    llvm-18 \
    gcc-10 \
    binutils \
    valgrind \
    cmake \
    git \
    autotools-dev \
    automake \
    autoconf \
    libtool \
    pkg-config \
    libcmocka-dev \
    libusb-1.0-0-dev \
    libudev-dev \
    libhidapi-dev

RUN apt-get update && apt-get install -y \
    doxygen \
    graphviz

# Dependencies of ARM Toolchain (specifically GDB)
RUN apt-get update && apt-get install -y \
    libncurses5

# Set gcc-10 as the default gcc
RUN update-alternatives --install /usr/bin/gcc gcc /usr/bin/gcc-10 100
RUN update-alternatives --install /usr/bin/gcov gcov /usr/bin/gcov-10 100

# Tools for CI
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    clang-format-18 \
    clang-tidy-18

RUN python3 -m pip install --upgrade pip

# Python modules
COPY py/bitbox02 /tmp/bitbox02
RUN python3 -m pip install /tmp/bitbox02
RUN rm -r /tmp/bitbox02
COPY py/requirements.txt /tmp
RUN python3 -m pip install --upgrade --requirement /tmp/requirements.txt
RUN rm /tmp/requirements.txt

# Python modules for CI
RUN python3 -m pip install --upgrade \
    pylint==2.13.9 \
    pylint-protobuf==0.20.2 \
    black==22.3.0 \
    mypy==0.960 \
    mypy-protobuf==3.2.0

# Python modules for packaging
RUN python3 -m pip install --upgrade \
    setuptools==41.2.0 \
    wheel==0.33.6 \
    twine==1.15.0

#Install protoc from release, because the version available on the repo is too old
RUN if [ "${TARGETPLATFORM}" = "linux/arm64" ]; then \
      PROTOC_URL=https://github.com/protocolbuffers/protobuf/releases/download/v21.2/protoc-21.2-linux-aarch_64.zip; \
    else \
      PROTOC_URL=https://github.com/protocolbuffers/protobuf/releases/download/v21.2/protoc-21.2-linux-x86_64.zip; \
    fi; \
    mkdir -p /opt/protoc && \
    curl -L0 ${PROTOC_URL} -o /tmp/protoc-21.2.zip && \
    unzip /tmp/protoc-21.2.zip -d /opt/protoc && \
    rm /tmp/protoc-21.2.zip
ENV PATH /opt/protoc/bin:$PATH

# Make Python3 the default
RUN update-alternatives --install /usr/bin/python python /usr/bin/python3 1

# Developer tools
RUN apt-get update && apt-get install -y \
    bash-completion
# Install gcovr from PIP to get a newer version than in apt repositories
RUN python3 -m pip install gcovr

# Install Go, used for the tools in tools/go and for test/gounittest
ENV GOPATH /opt/go
ENV GOROOT /opt/go_dist/go
ENV PATH $GOROOT/bin:$GOPATH/bin:$PATH
RUN mkdir -p /opt/go_dist && \
    curl https://dl.google.com/go/go1.19.3.linux-${TARGETARCH}.tar.gz | tar -xz -C /opt/go_dist

# Install lcov from release (the one from the repos is too old).
RUN cd /opt && wget https://github.com/linux-test-project/lcov/releases/download/v1.14/lcov-1.14.tar.gz && tar -xf lcov-1.14.tar.gz
ENV PATH /opt/lcov-1.14/bin:$PATH

# Install rust compiler
ENV PATH /opt/cargo/bin:$PATH
ENV RUSTUP_HOME=/opt/rustup
COPY src/rust/rust-toolchain.toml /tmp/rust-toolchain.toml
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | CARGO_HOME=/opt/cargo sh -s -- --default-toolchain $(grep -oP '(?<=channel = ")[^"]+' /tmp/rust-toolchain.toml) -y
RUN rustup target add thumbv7em-none-eabi
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN rustup component add rust-src
RUN CARGO_HOME=/opt/cargo cargo install cbindgen --version 0.28.0 --locked
RUN CARGO_HOME=/opt/cargo cargo install bindgen-cli --version 0.71.1 --locked

# Until cargo vendor supports vendoring dependencies of the rust std libs we
# need a copy of this file next to the toml file. It also has to be world
# writable so that invocations of `cargo vendor` can update it. Below is the
# tracking issue for `cargo vendor` to support rust std libs.
# https://github.com/rust-lang/wg-cargo-std-aware/issues/23
RUN cp "$(rustc --print=sysroot)/lib/rustlib/src/rust/library/Cargo.lock" "$(rustc --print=sysroot)/lib/rustlib/src/rust/library/test/"
RUN chmod 777 $(rustc --print=sysroot)/lib/rustlib/src/rust/library/test/Cargo.lock

COPY tools/prost-build-proto prost-build-proto
RUN CARGO_HOME=/opt/cargo cargo install --path prost-build-proto --locked

# Clean temporary files to reduce image size
RUN rm -rf /var/lib/apt/lists/*
