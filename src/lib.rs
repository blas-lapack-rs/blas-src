//! [BLAS] source of choice.
//!
//! The usage of the package is explained [here][usage].
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate] framework (macOS only),
//! * `netlib`, which is the reference one by [Netlib], and
//! * `openblas`, which is the one in [OpenBLAS].
//! * `intel-mkl`, which is the one in [Intel MKL].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! blas-src = { version = "0.1", features = ["accelerate"] }
//! blas-src = { version = "0.1", features = ["netlib"] }
//! blas-src = { version = "0.1", features = ["openblas"] }
//! blas-src = { version = "0.1", features = ["intel-mkl"] }
//! ```
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [blas]: https://en.wikipedia.org/wiki/BLAS
//! [netlib]: http://www.netlib.org/
//! [openblas]: http://www.openblas.net/
//! [intel mkl]: https://software.intel.com/en-us/mkl
//! [usage]: https://blas-lapack-rs.github.io/usage

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src as raw;
