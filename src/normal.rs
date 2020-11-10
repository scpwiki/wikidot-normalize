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

use crate::ascii;
use regex::Regex;
use trim_in_place::TrimInPlace;

lazy_static! {
    static ref NON_NORMAL: Regex = Regex::new(r"([^a-z0-9\-:_]").unwrap();
    static ref LEADING_UNDERSCORE: Regex = Regex::new(r"^_").unwrap();
    static ref LEADING_OR_TRAILING_DASHES: Regex = Regex::new(r"(^-+)|(-+$)").unwrap();
    static ref MULTIPLE_DASHES: Regex = Regex::new(r"-{2,}").unwrap();
    static ref MULTIPLE_COLONS: Regex = Regex::new(r":{2,}").unwrap();
    static ref COLON_DASH: Regex = Regex::new(r"(:-)|(-:)").unwrap();
    static ref UNDERSCORE_DASH: Regex = Regex::new(r"(_-)|(-_)").unwrap();
    static ref LEADING_OR_TRAILING_COLON: Regex = Regex::new(r"(^:)|(:$)").unwrap();
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
pub fn normalize(text: &mut String) {
    // Remove leading and trailing whitespace
    //
    // Note that stdlib .trim() is &str -> &str,
    // we want this to be in-place on a String.
    text.trim_in_place();

    // Transform latin-like characters into ASCII.
    // See ascii module for more details.
    ascii::transform_in_place(text);

    // Lowercase all ASCII alphabetic characters.
    // Combined with the previous transformation this should
    // lowercase every character we care about (and permit in normal form anyways).
    text.make_ascii_lowercase();

    // Run through the regular expression substitutions.
    replace_in_place(text, &*NON_NORMAL, "-");
    replace_in_place(text, &*LEADING_UNDERSCORE, ":_");
    // TODO "(?<!:)_" -> "-", negative look-behind, wtf lol
    // I think this means "the first underscore before any colons"
    replace_in_place(text, &*LEADING_OR_TRAILING_DASHES, "");
    replace_in_place(text, &*MULTIPLE_DASHES, "-");
    replace_in_place(text, &*MULTIPLE_COLONS, ":");
    replace_in_place(text, &*COLON_DASH, ":");
    replace_in_place(text, &*UNDERSCORE_DASH, "_");
    replace_in_place(text, &*LEADING_OR_TRAILING_COLON, "");
}

/// Determines if an arbitrary string is already in Wikidot normalized form.
pub fn is_normal(mut name: &str) -> bool {
    todo!()
}

fn replace_in_place(text: &mut String, regex: &Regex, replace_with: &str) {
    use regex::Captures;
    use std::ops::Range;

    fn get_range(captures: Captures) -> Range<usize> {
        let mtch = captures.get(0).unwrap();
        let start = mtch.start();
        let end = mtch.end();

        start..end
    }

    while let Some(captures) = regex.captures(text) {
        let range = get_range(captures);
        text.replace_range(range, replace_with);
    }
}
