// cargo readme > README.md (and remove the duplicated title)
//! <h1 align="center">unstd</h1>
//! <div align="center"><strong>
//!
//! A little utility belt.
//!
//! </strong></div><br />
//!
//! <div align="center">
//!   <!-- Crates version -->
//!   <a href="https://crates.io/crates/unstd">
//!     <img src="https://img.shields.io/crates/v/unstd.svg?style=flat-square"
//!     alt="Crates.io version" />
//!   </a>
//!   <!-- docs.rs -->
//!   <a href="https://docs.rs/unstd">
//!     <img src="https://img.shields.io/badge/docs.rs-latest-blue.svg?style=flat-square"
//!       alt="docs.rs docs" />
//!     <!-- <img src="https://docs.rs/unstd/badge.svg"
//!       alt="docs.rs docs" /> -->
//!   </a>
//!   <!-- CI -->
//!   <a href="https://crates.io/crates/unstd">
//!     <img src="https://img.shields.io/github/workflow/status/asaaki/unstd/CI/main?style=flat-square"
//!       alt="CI status" />
//!   </a>
//!   <!-- Downloads -->
//!   <a href="https://crates.io/crates/unstd">
//!     <img src="https://img.shields.io/crates/d/unstd.svg?style=flat-square"
//!       alt="Download" />
//!   </a>
//! </div>
//!
//! Tiny helpers which could be in `std::` of Rust, but aren't.
//!
//! ## License
//!
//! <sup>
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//! </sup>
//!
//! <br/>
//!
//! <sub>
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
//! be dual licensed as above, without any additional terms or conditions.
//! </sub>
//!

#![forbid(unsafe_code)]
#![cfg_attr(feature = "docs", feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(unused_imports)]
#![deny(missing_debug_implementations)]
#![warn(clippy::expect_used)]
#![deny(clippy::unwrap_used)]
#![deny(unused_results)]
#![doc(
    test(attr(allow(unused_variables), deny(warnings))),
    //html_logo_url = "https://raw.githubusercontent.com/asaaki/unstd/main/.assets/logo.svg"
)]

pub mod math;

#[cfg(test)]
mod tests {
    // other int types are tested via doc examples
    #[test]
    fn math_usize_divrem() {
        use super::math::usize::*;
        let actual = divrem(&3, &2);
        assert_eq!(actual, (1, 1))
    }
}
