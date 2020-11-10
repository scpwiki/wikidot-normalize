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

macro_rules! regex {
    ($name:tt, $expr:expr) => {
        lazy_static! {
            static ref $name: Regex = Regex::new($expr).unwrap();
        }
    };
}

regex!(NON_NORMAL, r"([^a-z0-9\-:_]");
regex!(LEADING_OR_TRAILING_DASHES, r"(^-+)|(-+$)");
regex!(MULTIPLE_DASHES, r"-{2,}");
regex!(MULTIPLE_COLONS, r":{2,}");
regex!(COLON_DASH, r"(:-)|(-:)");
regex!(UNDERSCORE_DASH, r"(_-)|(-_)");
regex!(LEADING_OR_TRAILING_COLON, r"(^:)|(:$)");

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
pub fn is_normal(name: &str) -> bool {
    todo!()
}

// Helpers

/// Manual implementation of the PCRE "(?<!:)_", which uses a negative look-behind.
///
/// Instead of a somewhat messy standard regex to replace it, it's easier
/// (and conveys the intent better) if I have a regular function do it instead.
///
/// This looks for underscores, provided it is not immediately preceded with a colon.
/// If such an underscore is found, it is replaced with a dash.
/// The goal is to replace underscores, excepting leading underscores for pages like
/// `_template`. The colon rule permits them to exist on categories, like `fragment:_template`.
///
/// Wikidot originally achieves this by prepending a colon to implicit `_default` category
/// slugs, making it avoid the underscore replacement.
///
/// That's dumb. This logic just makes an exception for the start of the string instead.
fn replace_underscores(text: &mut String) {
    let mut matches = vec![];
    let mut prev_colon = false;

    // Finding matching, non-conforming underscores
    for (idx, ch) in text.chars().enumerate() {
        if ch == '_' {
            // Allow a leading underscore at the start
            if idx == 0 {
                continue;
            }

            // Allow a leading underscore after a category
            if prev_colon {
                continue;
            }

            matches.push(idx);
        }

        prev_colon = ch == ':';
    }

    // Replace them with dashes
    for idx in matches {
        text.replace_range(idx..idx+1, "-");
    }
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
