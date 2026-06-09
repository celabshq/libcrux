#!/usr/bin/env bash

set -v -e -x

source $HOME/.profile

git clone https://github.com/AeneasVerif/eurydice.git
cd eurydice
git checkout $EURYDICE_REV

make setup-karamel
make setup-charon
make setup-libcrux

make test

# Remove Charon build files, since the binary has been generated
rm -rf charon/charon

echo "export KRML_HOME=$HOME/eurydice/karamel" >>$HOME/.profile
echo "export EURYDICE_HOME=$HOME/eurydice" >>$HOME/.profile
echo "export CHARON_HOME=$HOME/eurydice/charon" >>$HOME/.profile

CHARON_REV="$(jq -r ".nodes.charon.locked.rev" flake.lock)"
KRML_REV="$(jq -r ".nodes.karamel.locked.rev" flake.lock)"

echo "export KRML_REV=$KRML_REV" >>$HOME/.profile
echo "export EURYDICE_REV=$EURYDICE_REV" >>$HOME/.profile
echo "export CHARON_REV=$CHARON_REV" >>$HOME/.profile

source $HOME/.profile

opam clean
