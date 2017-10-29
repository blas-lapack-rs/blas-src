# blas-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The packages provides a [BLAS][blas] implementation of choice.

## Configuration

The following implementations are available:

* `accelerate`,
* `netlib`, and
* `openblas`.

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

[blas]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms

[build-img]: https://travis-ci.org/stainless-steel/blas-src.svg?branch=master
[build-url]: https://travis-ci.org/stainless-steel/blas-src
[documentation-img]: https://docs.rs/blas-src/badge.svg
[documentation-url]: https://docs.rs/blas-src
[package-img]: https://img.shields.io/crates/v/blas-src.svg
[package-url]: https://crates.io/crates/blas-src
