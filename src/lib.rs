// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: Apache-2.0

//! # Welcome!
//!
//! Telestes provides an implementation of Media Types; also formerly known
//! as MIME Types. This crate emphasizes performance and correctness; as well
//! as the ability to parse in a `const` context. It is `no_std`-enabled.
//!
//! To get started, check out the [`MediaType`] and [`Essence`] types.
//!
//! # Optional Features
//!
//! * `alloc` - enables the use of types in the `alloc` crate
//! * `serde` - enables serialization/deserialization using `serde`
//! * `db`    - enables the `const` database of IANA Media Types
//!
//! # Terminology
//!
//! The terminology of this crate should follow the terminology broadly used
//! in the RFCs or other notable documentation; with one exception. Since the
//! `type` terminology of a Media Type collides with the Rust `type` keyword,
//! we have chosen the terms `genus` and `species` to replace the terms for
//! `type` and `subtype`, respectively.
//!
//! # Example
//!
//! ```
//! use telestes::MediaType;
//!
//! let mt = MediaType::new("text/plain; charset=UTF-8").unwrap();
//! assert_eq!(mt, "text/plain; charset=UTF-8");
//! assert_eq!(mt.essence(), "text/plain");
//! assert_eq!(mt.essence().genus(), "text");
//! assert_eq!(mt.essence().species(), "plain");
//! assert_eq!(mt.parameters().get("charset"), Some("UTF-8"));
//! ```
//!
//! # Naming
//!
//! This crate is named after the character Telestes from the Greek play Seven
//! Against Thebes by Aeschylus. This character is possibly history's first
//! recorded pantomime.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rust_2018_idioms, unused_lifetimes, unused_qualifications)]
#![warn(clippy::all, clippy::panic)]
#![forbid(unsafe_code, clippy::expect_used)]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg_attr(docsrs, doc(cfg(feature = "db")))]
#[cfg(feature = "db")]
pub mod db;

mod essence;
mod mediatype;
mod params;
mod uncased;

pub use essence::Essence;
pub use mediatype::MediaType;
pub use params::Parameters;
pub use uncased::Uncased;

use core::ops::{Range, RangeFrom};

/// A parsing error
///
/// The error contains the byte offset of the offending character.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Error(pub usize);

/// A trait associating a Media Type with a Rust type
pub trait MediaTyped {
    const MEDIA_TYPE: MediaType<&'static str>;
}

struct Parsed {
    key: Range<usize>,
    val: Range<usize>,
    idx: RangeFrom<usize>,
}
