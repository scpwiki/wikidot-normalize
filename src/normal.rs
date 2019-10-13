/*
 * normal.rs
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

use regex::Regex;
use std::borrow::Cow;
use std::mem;
use std::str::Utf8Error;

lazy_static! {
    static ref NON_URL: Regex = Regex::new(r"([^\w/\-]+|-{2,})").unwrap();
    static ref START_DASHES: Regex = Regex::new(r"(^|/+)(?P<dash>-+)").unwrap();
    static ref END_DASHES: Regex = Regex::new(r"(?P<dash>-+)($|/+)").unwrap();
}

#[inline]
fn percent_decode(input: &str) -> Result<Cow<str>, Utf8Error> {
    use percent_encoding::percent_decode_str;

    percent_decode_str(input).decode_utf8()
}

/// Converts an arbitrary string into Wikidot normalized form, first decoding percent notation.
///
/// See [`normalize`] for information on normal form.
///
/// [`normalize`]: ./fn.normalize.html
pub fn normalize_decode(name: &mut String) {
    // Perform percent-decoding
    match percent_decode(&name) {
        Ok(Cow::Borrowed(_)) => (),
        Ok(Cow::Owned(mut decoded)) => mem::swap(name, &mut decoded),
        Err(_) => warn!("Error decoding percent string"),
    }

    normalize(name);
}

/// Converts an arbitrary string into Wikidot normalized form.
///
/// This will convert non-alphanumeric characters to dashes and
/// makes it lowercase.
///
/// Examples:
/// * `Big Cheese Horace` -> `big-cheese-horace`
/// * `bottom--Text` -> `bottom-text`
/// * `Tufto's Proposal` -> `tufto-s-proposal`
/// * `-test-` -> `test`
pub fn normalize(name: &mut String) {
    // Lowercase
    name.make_ascii_lowercase();

    // Convert non-URL characters to dashes
    while let Some(mtch) = NON_URL.find(name) {
        let start = mtch.start();
        let end = mtch.end();
        name.replace_range(start..end, "-");
    }

    // Remove leading and trailing dashes
    let get_range = |captures: regex::Captures| {
        let mtch = captures.name("dash").unwrap();
        let start = mtch.start();
        let end = mtch.end();

        start..end
    };

    while let Some(captures) = START_DASHES.captures(name) {
        let range = get_range(captures);
        name.replace_range(range, "");
    }

    while let Some(captures) = END_DASHES.captures(name) {
        let range = get_range(captures);
        name.replace_range(range, "");
    }
}

/// Determines if an arbitrary string is already in Wikidot normalized form.
///
/// If `slash` is true, the forward slash character is also accepted.
pub fn is_normal(name: &str, slash: bool) -> bool {
    // Is all lowercase
    let is_valid_char = |ch: char| -> bool {
        ch.is_ascii_lowercase()
            || ch.is_digit(10)
            || ch == ':'
            || ch == '-'
            || ch == '_'
            || (slash && ch == '/')
    };

    if !name.chars().all(is_valid_char) {
        return false;
    }

    // No special characters
    if NON_URL.find(name).is_some() {
        return false;
    }

    // Has leading or trailing dashes
    if START_DASHES.find(name).is_some() {
        return false;
    }

    if END_DASHES.find(name).is_some() {
        return false;
    }

    true
}
