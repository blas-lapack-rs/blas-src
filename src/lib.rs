//! [BLAS] source of choice.
//!
//! ## [Architecture]
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate] framework (macOS only),
//! * `blis`, which is the one in [BLIS],
//! * `intel-mkl`, which is the one in [Intel MKL],
//! * `netlib`, which is the reference one by [Netlib],
//! * `openblas`, which is the one in [OpenBLAS], and
//! * `r`, which is the one in [R].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! blas-src = { version = "0.8", features = ["accelerate"] }
//! blas-src = { version = "0.8", features = ["blis"] }
//! blas-src = { version = "0.8", features = ["intel-mkl"] }
//! blas-src = { version = "0.8", features = ["netlib"] }
//! blas-src = { version = "0.8", features = ["openblas"] }
//! blas-src = { version = "0.8", features = ["r"] }
//! ```
//!
//! [architecture]: https://blas-lapack-rs.github.io/architecture
//! [blas]: https://en.wikipedia.org/wiki/BLAS
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [blis]: https://github.com/flame/blis
//! [intel mkl]: https://software.intel.com/en-us/mkl
//! [netlib]: http://www.netlib.org/
//! [openblas]: http://www.openblas.net/
//! [R]: https://cran.r-project.org

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "blis")]
extern crate blis_src as raw;

#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

#[cfg(feature = "r")]
extern crate r_src as raw;
