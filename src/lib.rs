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

#![deny(missing_debug_implementations)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate percent_encoding;
extern crate regex;

#[cfg(test)]
#[macro_use]
extern crate str_macro;

mod normal;

#[cfg(test)]
mod test;

pub type StdResult<T, E> = std::result::Result<T, E>;

pub use self::normal::{is_normal, normalize, normalize_decode};

pub mod prelude {
    pub use super::normal::{is_normal, normalize, normalize_decode};
}
