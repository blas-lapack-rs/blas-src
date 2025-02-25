# blas-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The packages provides a [BLAS] source of choice.

## [Architecture]

## Configuration

The following implementations are available:

* `accelerate`, which is the one in the [Accelerate] framework (macOS only),
* `blis`, which is the one in [BLIS],
* `intel-mkl`, which is the one in [Intel MKL],
* `netlib`, which is the reference one by [Netlib],
* `openblas`, which is the one in [OpenBLAS], and
* `r`, which is the one in [R].

An implementation can be chosen as follows:

```toml
[dependencies]
blas-src = { version = "0.11", features = ["accelerate"] }
blas-src = { version = "0.11", features = ["blis"] }
blas-src = { version = "0.11", features = ["intel-mkl"] }
blas-src = { version = "0.11", features = ["netlib"] }
blas-src = { version = "0.11", features = ["openblas"] }
blas-src = { version = "0.11", features = ["r"] }
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[architecture]: https://blas-lapack-rs.github.io/architecture
[blas]: https://en.wikipedia.org/wiki/BLAS

[accelerate]: https://developer.apple.com/reference/accelerate
[blis]: https://github.com/flame/blis
[intel mkl]: https://software.intel.com/en-us/mkl
[netlib]: https://www.netlib.org/
[openblas]: https://www.openblas.net/
[r]: https://cran.r-project.org/

[build-img]: https://github.com/blas-lapack-rs/blas-src/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/blas-lapack-rs/blas-src/actions/workflows/build.yml
[documentation-img]: https://docs.rs/blas-src/badge.svg
[documentation-url]: https://docs.rs/blas-src
[package-img]: https://img.shields.io/crates/v/blas-src.svg
[package-url]: https://crates.io/crates/blas-src
