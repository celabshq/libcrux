#!/usr/bin/env bash

set -v -e -x

curl https://sh.rustup.rs -sSf | bash -s -- -y

# Prepare the sources
opam init --bare --disable-sandboxing --shell-setup --yes
opam switch create 5.1.1
echo "export FSTAR_HOME=$HOME/fstar" >>$HOME/.profile
echo "export HACL_HOME=$HOME/hacl-star" >>$HOME/.profile
echo "export HAX_HOME=$HOME/hax" >>$HOME/.profile
echo "export PATH=\"${PATH}:$HOME/fstar/bin:$HOME/z3/bin\"" >>$HOME/.profile
echo "[[ ! -r /home/$USER/.opam/opam-init/init.sh ]] || source /home/$USER/.opam/opam-init/init.sh  > /dev/null 2> /dev/null" >>$HOME/.profile

source $HOME/.profile
eval $(opam env)

# Get F*, HACL*, Charon, Karamel, Eurydice for running proofs and extraction
curl -L https://github.com/FStarLang/FStar/releases/download/v2025.02.17/fstar-v2025.02.17-Linux-x86_64.tar.gz \
    --output Fstar.tar.gz
tar --extract --file Fstar.tar.gz
rm -f Fstar.tar.gz

curl -L https://raw.githubusercontent.com/FStarLang/FStar/8080c2c10e2a15fdacea6df31f0921850294cd37/.scripts/get_fstar_z3.sh \
    --output get_fstar_z3.sh
chmod +x get_fstar_z3.sh
mkdir -p $HOME/z3/bin
./get_fstar_z3.sh $HOME/z3/bin
rm -rf get_fstar_z3.sh

curl -L https://github.com/hacl-star/hacl-star/archive/2a8b61343a1a7232611cb763b0dc3e4dff84d656.zip \
    --output hacl-star.zip
unzip hacl-star.zip
rm -rf hacl-star.zip
mv hacl-star-2a8b61343a1a7232611cb763b0dc3e4dff84d656/ hacl-star

opam install --yes ocamlfind visitors menhir ppx_deriving_yojson core_unix \
    sedlex wasm fix process pprint zarith yaml easy_logging terminal unionFind ocamlformat.0.27.0

