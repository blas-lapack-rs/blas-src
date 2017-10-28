//! [BLAS][blas] implementation of choice.
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`,
//! * `netlib`, and
//! * `openblas`.
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
//! [blas]: https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;
