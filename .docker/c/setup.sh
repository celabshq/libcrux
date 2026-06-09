#!/usr/bin/env bash

set -v -e -x

locale-gen $LANG
dpkg-reconfigure locales
useradd -d $HOME -s $SHELL -m $USER
echo "$USER:$USER" | chpasswd && adduser $USER sudo
