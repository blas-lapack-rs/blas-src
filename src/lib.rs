//! [BLAS] implementation of choice.
//!
//! Note that this package does not contain any functionality other than
//! compiling (if necessary) and linking to the chosen implementation. Bindings
//! are available in [`blas-sys`][crate-blas-sys], and wrappers are available in
//! [`blas`][crate-blas].
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate] framework (macOS only),
//! * `netlib`, which is the reference one by [Netlib], and
//! * `openblas`, which is the one in [OpenBLAS].
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
//! [crate-blas-sys]: https://crates.io/crates/blas-sys
//! [crate-blas]: https://crates.io/crates/blas
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [netlib]: http://www.netlib.org/
//! [openblas]: http://www.openblas.net/

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;
