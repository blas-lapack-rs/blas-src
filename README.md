# blas-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The packages provides a [BLAS][BLAS] implementation of choice.

Note that this package does not contain any functionality other than compiling
(if necessary) and linking to the chosen implementation. Bindings are available
in [`blas-sys`][blas-sys], and wrappers are available in [`blas`][blas].

## Configuration

The following implementations are available:

* `accelerate`, which is the one in the [Accelerate framework][accelerate] (macOS only),
* `netlib`, which is the reference one by [Netlib][netlib], and
* `openblas`, which is the one in [OpenBLAS][openblas].

An implementation can be chosen as follows:

```toml
[dependencies]
blas-src = { version = "0.1", features = ["accelerate"] }
blas-src = { version = "0.1", features = ["netlib"] }
blas-src = { version = "0.1", features = ["openblas"] }
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[BLAS]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
[blas-sys]: https://crates.io/crates/blas-sys
[blas]: https://crates.io/crates/blas

[accelerate]: https://developer.apple.com/reference/accelerate
[netlib]: http://www.netlib.org/
[openblas]: http://www.openblas.net/

[build-img]: https://travis-ci.org/stainless-steel/blas-src.svg?branch=master
[build-url]: https://travis-ci.org/stainless-steel/blas-src
[documentation-img]: https://docs.rs/blas-src/badge.svg
[documentation-url]: https://docs.rs/blas-src
[package-img]: https://img.shields.io/crates/v/blas-src.svg
[package-url]: https://crates.io/crates/blas-src
