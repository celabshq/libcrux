# This Dockerfile should be run from this directory
# `docker build . -t franziskus/libcrux-c`

FROM ubuntu:24.04
LABEL maintainer="Franziskus Kiefer <franziskuskiefer@gmail.com>"

ENV DEBIAN_FRONTEND=noninteractive
ENV SHELL=/bin/bash
ENV USER=user
ENV LOGNAME=$USER
ENV HOME=/home/$USER
ENV LANG=en_US.UTF-8
ENV LC_ALL=$LANG
ENV PATH="$HOME/.cargo/bin:${PATH}"


# Install dependencies.
RUN apt-get -y update && apt-get install -y \
    build-essential \
    opam \
    jq \
    git \
    clang \
    ninja-build \
    libgmp-dev \
    locales \
    pkg-config \
    clang-format\
    cmake \
    curl

ADD setup.sh /tmp/setup.sh
RUN bash /tmp/setup.sh

WORKDIR $HOME
USER $USER

ENV FSTAR_HOME=$HOME/fstar
ENV HACL_HOME=$HOME/hacl-star
ENV EURYDICE_HOME=$HOME/eurydice
ENV KRML_HOME=$HOME/eurydice/karamel
ENV CHARON_HOME=$HOME/eurydice/charon
ENV EURYDICE_REV=aaa9fa657fb6f09802edb890252040d94cd93982
ENV PATH="${PATH}:$HOME/fstar/bin:$HOME/z3/bin"
ENV CC=clang
ENV CXX=clang++

# Setup & install.
ADD install.sh /tmp/install.sh
RUN bash /tmp/install.sh
ADD ext-tools.sh /tmp/ext-tools.sh
RUN bash /tmp/ext-tools.sh
