# hax (cargo-hax + Rust/OCaml engines) and ProVerif, at a selectable hax version.
#
# Build:
#   docker build -t hax:0.5.0 --build-arg HAX_VERSION=0.5.0 .docker/hax
#
# Use with a libcrux checkout mounted at /work, e.g. the PSQ ProVerif check:
#   docker run --rm -it -v "$PWD":/work hax:0.5.0 \
#     bash libcrux-psq/proofs/proverif/check-pv.sh

FROM ocaml/opam:ubuntu-24.04-ocaml-5.4

ARG HAX_VERSION=0.5.0
ARG HAX_REF=cargo-hax-v${HAX_VERSION}

# libgtk2.0-dev: ProVerif depends on lablgtk.
RUN sudo apt-get update \
    && sudo apt-get install -y --no-install-recommends \
        curl jq nodejs python3 pkg-config libgtk2.0-dev \
    && sudo rm -rf /var/lib/apt/lists/*

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain none
ENV PATH=/home/opam/.cargo/bin:$PATH

RUN git clone --depth 1 --branch "$HAX_REF" https://github.com/cryspen/hax /home/opam/hax
WORKDIR /home/opam/hax

# setup.sh insists on an exact OCaml version; take it from the checkout so the
# image follows whatever HAX_VERSION requires.
RUN opam update \
    && opam install -y "ocaml-base-compiler.$(sed -n 's/^REQUIRED_OCAML_VERSION="\(.*\)"/\1/p' setup.sh)" \
    && opam install -y proverif \
    && rustup default "$(sed -n 's/^channel = "\(.*\)"/\1/p' rust-toolchain.toml)" \
    && opam exec -- ./setup.sh -j "$(nproc)" \
    && rm -rf target \
    && opam clean -a -c -s --logs

# check-pv.sh reads the ProVerif proof-libs from here.
ENV HAX_PROVERIF_DIR=/home/opam/hax
# Keep the container's Linux build artifacts away from the host's target/.
ENV CARGO_TARGET_DIR=/work/target/docker-hax

WORKDIR /work
ENTRYPOINT ["opam", "exec", "--"]
CMD ["bash"]
