# blas-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The packages provides a [BLAS] source of choice.

The usage of the package is explained [here][usage].

## Configuration

The following implementations are available:

* `accelerate`, which is the one in the [Accelerate] framework (macOS only),
* `intel-mkl`, which is the one in [Intel MKL],
* `netlib`, which is the reference one by [Netlib], and
* `openblas`, which is the one in [OpenBLAS].

An implementation can be chosen as follows:

```toml
[dependencies]
blas-src = { version = "0.6", features = ["accelerate"] }
blas-src = { version = "0.6", features = ["intel-mkl"] }
blas-src = { version = "0.6", features = ["netlib"] }
blas-src = { version = "0.6", features = ["openblas"] }
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[accelerate]: https://developer.apple.com/reference/accelerate
[blas]: https://en.wikipedia.org/wiki/BLAS
[intel mkl]: https://software.intel.com/en-us/mkl
[netlib]: http://www.netlib.org/
[openblas]: http://www.openblas.net/
[usage]: https://blas-lapack-rs.github.io/usage

[build-img]: https://travis-ci.org/blas-lapack-rs/blas-src.svg?branch=master
[build-url]: https://travis-ci.org/blas-lapack-rs/blas-src
[documentation-img]: https://docs.rs/blas-src/badge.svg
[documentation-url]: https://docs.rs/blas-src
[package-img]: https://img.shields.io/crates/v/blas-src.svg
[package-url]: https://crates.io/crates/blas-src
