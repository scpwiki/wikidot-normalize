/*
 * lib.rs
 *
 * wikidot-normalize - Library to provide Wikidot-compatible normalization.
 * Copyright (c) 2019-2021 Ammon Smith
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
//! Essentially, only alphanumeric characters, and `:`, `-`, and `_` are permitted.
//!
//! * Any uppercase characters are case folded, and any characters outside
//! the above set are collapsed into dashes. Multiple dashes or forward slashes are compressed
//! into a single instance.
//! * Any trailing forward slashes are stripped.
//! * Finally, any leading, trailing, or multiple dashes are removed.

#[macro_use]
extern crate lazy_static;
extern crate maplit;
extern crate regex;

#[cfg(test)]
#[macro_use]
extern crate str_macro;
extern crate trim_in_place;
extern crate unicode_normalization;

mod normal;
mod underscore;
mod unicode;

pub use self::normal::normalize;
