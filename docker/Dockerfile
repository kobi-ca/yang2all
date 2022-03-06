FROM ubuntu:18.04

# defaults
RUN \
    apt-get update && apt-get install -y \
    git \
    curl \
    wget \
    libssl-dev \
    libtool \
    build-essential \
    vim \
    autoconf \
    automake \
    pkg-config \
    libgtk-3-dev \
    make \
    valgrind \
    doxygen \
    libev-dev \
    libpcre3-dev \
    unzip \
    sudo \
    python3 \
    bison \
    flex \
    swig \
    libcmocka0 \
    libcmocka-dev \
    cmake \
    supervisor \
    doxygen \
    libpcre3-dev

WORKDIR /tmp/
RUN git clone https://github.com/CESNET/libyang.git && \
    cd libyang && \
    mkdir build && \
    cd build && \
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_VERBOSE_MAKEFILE:BOOL=ON -DENABLE_TESTS=ON .. && \
    make && \
    make test && \
    make install
