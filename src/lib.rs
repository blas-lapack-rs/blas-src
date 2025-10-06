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
//! blas-src = { version = "0.14", features = ["accelerate"] }
//! blas-src = { version = "0.14", features = ["blis"] }
//! blas-src = { version = "0.14", features = ["intel-mkl-dynamic-parallel"] }
//! blas-src = { version = "0.14", features = ["intel-mkl-dynamic-sequential"] }
//! blas-src = { version = "0.14", features = ["intel-mkl-static-parallel"] }
//! blas-src = { version = "0.14", features = ["intel-mkl-static-sequential"] }
//! blas-src = { version = "0.14", features = ["netlib"] }
//! blas-src = { version = "0.14", features = ["openblas"] }
//! blas-src = { version = "0.14", features = ["r"] }
//! ```
//!
//! [architecture]: https://blas-lapack-rs.github.io/architecture
//! [blas]: https://en.wikipedia.org/wiki/BLAS
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [blis]: https://github.com/flame/blis
//! [intel mkl]: https://software.intel.com/en-us/mkl
//! [netlib]: https://www.netlib.org/
//! [openblas]: https://www.openblas.net/
//! [r]: https://cran.r-project.org/

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "blis")]
extern crate blis_src as raw;

#[cfg(any(
    feature = "intel-mkl-dynamic-parallel",
    feature = "intel-mkl-dynamic-sequential",
    feature = "intel-mkl-static-parallel",
    feature = "intel-mkl-static-sequential",
))]
extern crate intel_mkl_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

#[cfg(feature = "r")]
extern crate r_src as raw;
