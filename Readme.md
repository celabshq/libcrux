# libcrux - a high-assurance cryptographic library in Rust

libcrux is a cryptographic library written in Rust. It uses formally verified
code generated from the [HACL* project](https://github.com/hacl-star/hacl-star) and includes Rust code that is
directly verified for runtime safety and functional correctness using the
[hax toolchain](https://github.com/cryspen/hax). For more details, see below.

libcrux is in pre-release (all of its crates are versioned < `0.1`). If you wish to use any of these crates
in production, get in touch with the [maintainers](mailto:info@cryspen.com) and we can advise you on whether
libcrux is a good fit for your use-case.

## Minimum Supported Rust Version (MSRV)

The default feature set has a MSRV of `1.78.0`. `no_std` environments
are supported starting from Rust version `1.81.0`.

## Randomness

libcrux provides a DRBG implementation that can be used standalone (`drbg::Drbg`)
or through the `Rng` traits.

## `no_std` support
`libcrux` and the individual primitive crates it depends on support
`no_std` environments given a global allocator for the target
platform.

## Verification status

As a quick indicator of overall verification status, subcrates in this workspace include the following badges:

- ![pre-verification] to indicate that most (or all) of the code that
  is contained in default features of that crate is not (yet)
  verified.
- ![verified-hacl] to indicate that algorithms in a crate have been verified and extracted to Rust as part of the HACL* project. The source F* code in HACL* is verified for memory safety, functional correctness against a high-level spec, and secret independence (for details, see [these](https://eprint.iacr.org/2017/536) [papers](https://arxiv.org/abs/1703.00053).) Top-level Rust APIs in these crates accessing the code from HACL* may not be verified.

- ![verified] to indicate that most (or all) of the Rust code that is contained in the default feature set is verified using the [hax toolchain](https://github.com/cryspen/hax). Where indicated, the code is verified to be panic free and functionally correct against a mathematical spec (for an overview of hax, see [this paper](https://eprint.iacr.org/2025/142))

Importantly, executables compiled from the code in this repository are *not* verified to be side-channel resistant, although we try to enforce that the source code is secret-independent (also sometimes called “constant-time”).

In every case, please refer to the more detailed notes on verification in each sub-crate to learn more about what has (or has not) been verified in the particular case, and [reach out to us](mailto:info@cryspen.com) if you have any questions.

## Publications

Libcrux was introduced in this article at RWC 2023. The verification methodology used to build libcrux is described in [this paper](https://eprint.iacr.org/2025/142).

[architecture]: ./Architecture.md
[hacspec]: https://hacspec.org
[formal verification]: ./formal_verification

[verified-hacl]: https://img.shields.io/badge/verified%20(hacl--rs)-brightgreen?style=for-the-badge&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz48IS0tIFVwbG9hZGVkIHRvOiBTVkcgUmVwbywgd3d3LnN2Z3JlcG8uY29tLCBHZW5lcmF0b3I6IFNWRyBSZXBvIE1peGVyIFRvb2xzIC0tPg0KPHN2ZyB3aWR0aD0iODAwcHgiIGhlaWdodD0iODAwcHgiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4NCjxwYXRoIGQ9Ik05IDEyTDExIDE0TDE1IDkuOTk5OTlNMjAgMTJDMjAgMTYuNDYxMSAxNC41NCAxOS42OTM3IDEyLjY0MTQgMjAuNjgzQzEyLjQzNjEgMjAuNzkgMTIuMzMzNCAyMC44NDM1IDEyLjE5MSAyMC44NzEyQzEyLjA4IDIwLjg5MjggMTEuOTIgMjAuODkyOCAxMS44MDkgMjAuODcxMkMxMS42NjY2IDIwLjg0MzUgMTEuNTYzOSAyMC43OSAxMS4zNTg2IDIwLjY4M0M5LjQ1OTk2IDE5LjY5MzcgNCAxNi40NjExIDQgMTJWOC4yMTc1OUM0IDcuNDE4MDggNCA3LjAxODMzIDQuMTMwNzYgNi42NzQ3QzQuMjQ2MjcgNi4zNzExMyA0LjQzMzk4IDYuMTAwMjcgNC42Nzc2NiA1Ljg4NTUyQzQuOTUzNSA1LjY0MjQzIDUuMzI3OCA1LjUwMjA3IDYuMDc2NCA1LjIyMTM0TDExLjQzODIgMy4yMTA2N0MxMS42NDYxIDMuMTMyNzEgMTEuNzUgMy4wOTM3MyAxMS44NTcgMy4wNzgyN0MxMS45NTE4IDMuMDY0NTcgMTIuMDQ4MiAzLjA2NDU3IDEyLjE0MyAzLjA3ODI3QzEyLjI1IDMuMDkzNzMgMTIuMzUzOSAzLjEzMjcxIDEyLjU2MTggMy4yMTA2N0wxNy45MjM2IDUuMjIxMzRDMTguNjcyMiA1LjUwMjA3IDE5LjA0NjUgNS42NDI0MyAxOS4zMjIzIDUuODg1NTJDMTkuNTY2IDYuMTAwMjcgMTkuNzUzNyA2LjM3MTEzIDE5Ljg2OTIgNi42NzQ3QzIwIDcuMDE4MzMgMjAgNy40MTgwOCAyMCA4LjIxNzU5VjEyWiIgc3Ryb2tlPSIjMDAwMDAwIiBzdHJva2Utd2lkdGg9IjIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIvPg0KPC9zdmc%2B
[verified]: https://img.shields.io/badge/verified-brightgreen.svg?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz48IS0tIFVwbG9hZGVkIHRvOiBTVkcgUmVwbywgd3d3LnN2Z3JlcG8uY29tLCBHZW5lcmF0b3I6IFNWRyBSZXBvIE1peGVyIFRvb2xzIC0tPg0KPHN2ZyB3aWR0aD0iODAwcHgiIGhlaWdodD0iODAwcHgiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4NCjxwYXRoIGQ9Ik05IDEyTDExIDE0TDE1IDkuOTk5OTlNMjAgMTJDMjAgMTYuNDYxMSAxNC41NCAxOS42OTM3IDEyLjY0MTQgMjAuNjgzQzEyLjQzNjEgMjAuNzkgMTIuMzMzNCAyMC44NDM1IDEyLjE5MSAyMC44NzEyQzEyLjA4IDIwLjg5MjggMTEuOTIgMjAuODkyOCAxMS44MDkgMjAuODcxMkMxMS42NjY2IDIwLjg0MzUgMTEuNTYzOSAyMC43OSAxMS4zNTg2IDIwLjY4M0M5LjQ1OTk2IDE5LjY5MzcgNCAxNi40NjExIDQgMTJWOC4yMTc1OUM0IDcuNDE4MDggNCA3LjAxODMzIDQuMTMwNzYgNi42NzQ3QzQuMjQ2MjcgNi4zNzExMyA0LjQzMzk4IDYuMTAwMjcgNC42Nzc2NiA1Ljg4NTUyQzQuOTUzNSA1LjY0MjQzIDUuMzI3OCA1LjUwMjA3IDYuMDc2NCA1LjIyMTM0TDExLjQzODIgMy4yMTA2N0MxMS42NDYxIDMuMTMyNzEgMTEuNzUgMy4wOTM3MyAxMS44NTcgMy4wNzgyN0MxMS45NTE4IDMuMDY0NTcgMTIuMDQ4MiAzLjA2NDU3IDEyLjE0MyAzLjA3ODI3QzEyLjI1IDMuMDkzNzMgMTIuMzUzOSAzLjEzMjcxIDEyLjU2MTggMy4yMTA2N0wxNy45MjM2IDUuMjIxMzRDMTguNjcyMiA1LjUwMjA3IDE5LjA0NjUgNS42NDI0MyAxOS4zMjIzIDUuODg1NTJDMTkuNTY2IDYuMTAwMjcgMTkuNzUzNyA2LjM3MTEzIDE5Ljg2OTIgNi42NzQ3QzIwIDcuMDE4MzMgMjAgNy40MTgwOCAyMCA4LjIxNzU5VjEyWiIgc3Ryb2tlPSIjMDAwMDAwIiBzdHJva2Utd2lkdGg9IjIiIHN0cm9rZS1saW5lY2FwPSJyb3VuZCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIvPg0KPC9zdmc+
[pre-verification]: https://img.shields.io/badge/pre_verification-orange.svg?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz48IS0tIFVwbG9hZGVkIHRvOiBTVkcgUmVwbywgd3d3LnN2Z3JlcG8uY29tLCBHZW5lcmF0b3I6IFNWRyBSZXBvIE1peGVyIFRvb2xzIC0tPg0KPHN2ZyB3aWR0aD0iODAwcHgiIGhlaWdodD0iODAwcHgiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4NCjxwYXRoIGQ9Ik05IDEySDE1TTIwIDEyQzIwIDE2LjQ2MTEgMTQuNTQgMTkuNjkzNyAxMi42NDE0IDIwLjY4M0MxMi40MzYxIDIwLjc5IDEyLjMzMzQgMjAuODQzNSAxMi4xOTEgMjAuODcxMkMxMi4wOCAyMC44OTI4IDExLjkyIDIwLjg5MjggMTEuODA5IDIwLjg3MTJDMTEuNjY2NiAyMC44NDM1IDExLjU2MzkgMjAuNzkgMTEuMzU4NiAyMC42ODNDOS40NTk5NiAxOS42OTM3IDQgMTYuNDYxMSA0IDEyVjguMjE3NTlDNCA3LjQxODA4IDQgNy4wMTgzMyA0LjEzMDc2IDYuNjc0N0M0LjI0NjI3IDYuMzcxMTMgNC40MzM5OCA2LjEwMDI3IDQuNjc3NjYgNS44ODU1MkM0Ljk1MzUgNS42NDI0MyA1LjMyNzggNS41MDIwNyA2LjA3NjQgNS4yMjEzNEwxMS40MzgyIDMuMjEwNjdDMTEuNjQ2MSAzLjEzMjcxIDExLjc1IDMuMDkzNzMgMTEuODU3IDMuMDc4MjdDMTEuOTUxOCAzLjA2NDU3IDEyLjA0ODIgMy4wNjQ1NyAxMi4xNDMgMy4wNzgyN0MxMi4yNSAzLjA5MzczIDEyLjM1MzkgMy4xMzI3MSAxMi41NjE4IDMuMjEwNjdMMTcuOTIzNiA1LjIyMTM0QzE4LjY3MjIgNS41MDIwNyAxOS4wNDY1IDUuNjQyNDMgMTkuMzIyMyA1Ljg4NTUyQzE5LjU2NiA2LjEwMDI3IDE5Ljc1MzcgNi4zNzExMyAxOS44NjkyIDYuNjc0N0MyMCA3LjAxODMzIDIwIDcuNDE4MDggMjAgOC4yMTc1OVYxMloiIHN0cm9rZT0iIzAwMDAwMCIgc3Ryb2tlLXdpZHRoPSIyIiBzdHJva2UtbGluZWNhcD0icm91bmQiIHN0cm9rZS1saW5lam9pbj0icm91bmQiLz4NCjwvc3ZnPg==
