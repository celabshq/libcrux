# Working with Libcrux ML-KEM

This directory contains Rust code for ML-KEM that can be formally verified using
the [hax toolchain](https://github.com/cryspen/hax) and can be compiled to C/C++
using the [eurydice toolchain](https://github.com/AeneasVerif/eurydice).

In this documemt, we describe how to install the tools that will allow you to
work with these tools. 
The following instructions have been tested with Debian (stable or testing on x64), 
Ubuntu 22.04 (on x64), and MacOS (aarch64). Variations of these instructions should
work on most Linux distributions.

For installing the tools and running the proofs, we recommend a machine with 
at least 4 cores, 16GB of memory, and 128GB of disk space.

## Prerequisites

The machine must have installed the following:
* Gcc >= 13 or Clang >= 14
* Rustup (cargo, rustc)
* Opam (with ocaml version 5.1.1)
* Nodejs, jq
* clang-format-18, cmake, ninja-build

The C compiler version constraints come from the Gtest and benchmarking framework;
our C code compiles with all standard compiler versions.

If you have a machine with all of the above, you can skip to the instructions 
for our toolchain. 

### Install Rust, OCaml, C tools

In a Debian testing environment, the following should install the pre-requisites.
The main notable step here is that we require the OCaml package manager (opam) installed
with the OCaml version set to 5.1.1. (Other OCaml versions may also work, but we have
tested extensively on this version of OCaml.)

Note that the sandboxing of opam must be disabled when running in Docker.
Answer yes when asked, or call `opam init` with `--disable-sandboxing -y`.

```bash
sudo apt -y install gcc git rustup nodejs jq opam clang-format-18 cmake ninja-build libgmp-dev pkg-config libffi-dev curl 

opam init
opam switch create 5.1.1
eval $(opam env --switch=5.1.1)

rustup default stable
export PATH=$PATH:$HOME/.cargo/bin
```

## Install Verification Tools (Hax, F*, Z3)

The following instructions should install all the verification tools needed by ML-KEM.
The main notable step is that F* requires two specific versions of Z3 to be installed.
Future versions of F* are expected to need only one (typically the latest) version of Z3.

```bash
# Install hax
git clone https://github.com/cryspen/hax.git
cd hax
./setup.sh

# Install F*
opam --yes install fstar
eval $(opam env)

# Install Z3 versions for F*
wget https://raw.githubusercontent.com/FStarLang/FStar/refs/heads/master/.scripts/get_fstar_z3.sh
chmod +x get_fstar_z3.sh
sudo ./get_fstar_z3.sh /usr/local/bin
```

## Install Compilation Tools (Charon, KaRaMeL, Eurydice)

To compile our Rust code to C, we need to install Eurydice and its dependencies.
Currently, we build and install these tools from source, as shown below.
In future, we plan to package the installation into a more convenient script.

```bash
# needed opam packages
opam install ppx_deriving_yojson zarith pprint "menhir>=20161115" sedlex process fix "wasm>=2.0.0" visitors ctypes-foreign ctypes uucp terminal yaml unionFind easy_logging

# install charon
git clone https://github.com/AeneasVerif/charon
cd charon
make -j

cd ..
# install karamel
git clone https://github.com/FStarLang/karamel.git
cd karamel
make -j

cd ..
# install eurydice
git clone https://github.com/AeneasVerif/eurydice.git
cd eurydice/lib
ln -s ~/charon charon
ln -s ~/karamel/lib krml
cd ..
make -j
```

## Testing the Tools

### Setting Environment Variables

After installing the tools, you may want to add the following environment variables
into your `.profile` or equivalent so that they are set every time you log in.

```bash
# To be set in any shell where we run the tools
eval $(opam env --switch=5.1.1)
export PATH=$PATH:$HOME/.cargo/bin
export KRML_HOME=~/karamel
export CHARON_HOME=~/charon
export EURYDICE_HOME=~/eurydice
```

### Testing ML-KEM in Rust

We encourage developers to always begin by testing the code in Rust before
doing any verification or C compilation:

```bash
git clone https://github.com/cryspen/libcrux.git
cd libcrux/libcrux-ml-kem
cargo test
cargo bench
```

### Compiling ML-KEM from Rust to C

We provide a script `boring.sh` to compile the Rust code for ML-KEM 768 to C.
You can use this script to regenerate the C code and test and benchmark it.

```bash
cd libcrux/libcrux-ml-kem/extracts/cpp_header_only
./extract.sh --no-clean
cd generated
export LIBCRUX_BENCHMARKS=1
cmake -B build -G "Ninja Multi-Config"
cmake --build build
./build/Debug/ml_kem_test

cmake --build build --config Release
./build/Release/ml_kem_test
./build/Release/ml_kem_bench
```

Note that the C code is broken into several files:
* `cg/libcrux_mlkem768_portable.h`: A portable implementation of ML-KEM 768
* `cg/libcrux_mlkem768_avx2.h`: An AVX2 implementation of ML-KEM 768
* `cg/libcrux_ct_ops.h`: Constant-time operations used in our code
* `cg/libcrux_mlkem_core.h`: Types and functions used by both portable and AVX2 implementations
* `cg/eurydice_glue.h`: C translations of a subset of the Rust core library
* `cg/libcrux_sha3_portable.h`: A portable implementation of SHA-3 (including KeccakX4)
* `cg/libcrux_sha3_avx2.h`: An AVX2 implementation of SHA-3 (including KeccakX4)


### Verifying ML-KEM using hax

We provide a python script called `hax.py` that compiles the Rust code to F* and
formally verifies it.

```bash
cd libcrux/libcrux-ml-kem
./hax.py extract
./hax.py prove
```

The F* files are located in `libcrux-ml-kem/proofs/fstar/extraction` and the spec they are verified against is in `libcrux-ml-kem/proofs/fstar/spec`.

### Modifying the Top-Level API

The main API exposed by the ML-KEM implementation (e.g. in
`cg/libcrux_mlkem768_portable.h`, `cg/libcrux_mlkem768_avx2.h`)
consists of the key generation, encapsulation, and decapsulation
functions, in both a packed and unpacked style.

We anticipate that libraries that use Libcrux may often want to change
this API or add new alternatives, and so show how this can be done.
Note that these API functions themselves must obey the pre-conditions
imposed by the inner (formally verified) functions of the
implementation such as the code in the IND-CCA and IND-CPA modules. 

Here is a diff that adds a simple mutable wrapper around the current
ML-KEM encapsulate function:

```diff
diff --git a/libcrux-ml-kem/src/mlkem768.rs b/libcrux-ml-kem/src/mlkem768.rs
index d3bd4b1c..e9a6663d 100644
--- a/libcrux-ml-kem/src/mlkem768.rs
+++ b/libcrux-ml-kem/src/mlkem768.rs
@@ -142,6 +142,32 @@ macro_rules! instantiate {
                 >(public_key, &randomness)
             }

+            /// Mutable version of encap
+            pub fn encapsulate_mut(
+                public_key: &MlKem768PublicKey,
+                randomness: [u8; SHARED_SECRET_SIZE],
+                ciphertext: &mut MlKem768Ciphertext,
+                shared_secret: &mut MlKemSharedSecret
+            ) {
+                let (c,ss) = p::encapsulate::<
+                    RANK,
+                    CPA_PKE_CIPHERTEXT_SIZE,
+                    CPA_PKE_PUBLIC_KEY_SIZE,
+                    T_AS_NTT_ENCODED_SIZE,
+                    C1_SIZE,
+                    C2_SIZE,
+                    VECTOR_U_COMPRESSION_FACTOR,
+                    VECTOR_V_COMPRESSION_FACTOR,
+                    C1_BLOCK_SIZE,
+                    ETA1,
+                    ETA1_RANDOMNESS_SIZE,
+                    ETA2,
+                    ETA2_RANDOMNESS_SIZE,
+                >(public_key, randomness);
+                *ciphertext = c;
+                *shared_secret = ss;
+            }
+
             /// Encapsulate Kyber 768
             ///
             /// Generates an ([`MlKem768Ciphertext`], [`MlKemSharedSecret`]) tuple.
```

After making this change, you can rerun the Rust cargo tests, re-verify the code,
and re-extract the C code.

```bash
cargo test
./hax.py extract
./hax.py prove
(cd extracts/cpp_header_only; ./extract.sh --no-clean)
```

Note that this function must obey the pre-conditions of the inner `p::encapsulate` function
so if you make a mistake in one of the const-generic functions, the verification will fail.

### Modifying the Constant-Time Operations

Another module that may often require defensive edits against compiler optimizations
is `constant_time_ops` (translated to `cg/libcrux_ct_ops.h`). For example, this
module currently contains an `inz` function that returns 0 if the input byte is 0
and 1 otherwise. The current code implements this function and verified it in one
way, but you can replace it with a different implementation, e.g.:

```rust
#[inline(never)] // Don't inline this to avoid that the compiler optimizes this out.
#[cfg_attr(hax, hax_lib::ensures(|result| if value == 0 {result == 0} else {result == 1}))]
fn inz2(value: u8) -> u8 {
    let value = value as i32;
    let result = ((-value) as u32) >> 31;
    let res = result as u8;
    res
}
```

Again, you can test the Rust, verify it with hax, and compile it to C.

```bash
cargo test
./hax.py extract
./hax.py prove
(cd extracts/cpp_header_only; ./extract.sh --no-clean)
```

Note that the post-condition (`ensures`) captures its functional specification
and hence hax (and F*) will formally prove that this function is correct.
If you mess up the masking in the function, the verification will fail.

## Ongoing Improvements

We are currently working on the following improvements to make the above workflow smoother:
* An installation script that installs all the tools above
* Making the formal specifications more readable by rewriting them from F* to Rust
* Making the proofs and compilation process more robust to bigger changes in the source code






