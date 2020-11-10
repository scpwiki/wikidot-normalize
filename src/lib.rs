/*
 * lib.rs
 *
 * wikidot-normalize - Library to provide Wikidot-compatible normalization.
 * Copyright (c) 2019 Ammon Smith
 *
 * wikidot-normalize is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

#![deny(missing_debug_implementations, missing_docs)]

//! A library to provide Wikidot-compatible string normalization.
//!
//! Wikidot ensures all names of pages subscribe to a particular pattern.
//! Essentially, only the characters `:`, `a-z`, `0-9`, and `-` can be outputted.
//! Any uppercase ASCII characters are made lowercase, and any characters outside
//! the above set are collapsed into dashes. Multiple dashes or forward slashes are compressed
//! into a single instance. Any trailing forward slashes are stripped.
//! Finally, any leading, trailing, or multiple dashes are removed.

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[cfg(test)]
#[macro_use]
extern crate str_macro;

mod normal;

#[cfg(test)]
mod test;

pub use self::normal::{is_normal, normalize};

/// A "prelude" for consumers of the `wikidot-normalize` crate.
///
/// This prelude includes all exports from the crate, and is provided
/// for convenience without requiring programs to do a glob import of
/// the whole crate.
pub mod prelude {
    pub use super::normal::{is_normal, normalize};
}
