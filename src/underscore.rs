/*
 * underscore.rs
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
pub fn replace_underscores(text: &mut String) {
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

#[test]
fn test_replace_underscores() {
}
