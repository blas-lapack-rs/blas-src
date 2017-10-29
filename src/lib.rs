//! [BLAS][blas] implementation of choice.
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate framework][accelerate] (macOS only),
//! * `netlib`, which is the reference one by [Netlib][netlib], and
//! * `openblas`, which is the one in [OpenBLAS][openblas].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! blas-src = { version = "0.1", features = ["accelerate"] }
//! blas-src = { version = "0.1", features = ["netlib"] }
//! blas-src = { version = "0.1", features = ["openblas"] }
//! ```
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [blas]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms
//! [netlib]: http://www.netlib.org/
//! [openblas]: http://www.openblas.net/

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;
