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
use crate::underscore::replace_underscores;
use regex::Regex;
use trim_in_place::TrimInPlace;

macro_rules! regex {
    ($name:tt, $expr:expr) => {
        lazy_static! {
            static ref $name: Regex = Regex::new($expr).unwrap();
        }
    };
}

regex!(NON_NORMAL, r"[^a-z0-9\-:_]");
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

    // Replace all characters not allowed in normal form.
    replace_in_place(text, &*NON_NORMAL, "-");

    // Replace non-leading underscores with dashes.
    //
    // Permits names like "_template" or "category:_template".
    replace_underscores(text);

    // Remove any leading or trailing dashes.
    replace_in_place(text, &*LEADING_OR_TRAILING_DASHES, "");

    // Merge multiple dashes and colons into one.
    replace_in_place(text, &*MULTIPLE_DASHES, "-");
    replace_in_place(text, &*MULTIPLE_COLONS, ":");

    // Remove any leading or trailing dashes next to colons or underscores.
    replace_in_place(text, &*COLON_DASH, ":");
    replace_in_place(text, &*UNDERSCORE_DASH, "_");

    // Remove any leading or trailing colons.
    replace_in_place(text, &*LEADING_OR_TRAILING_COLON, "");
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

#[test]
fn test_normalize() {
    macro_rules! check {
        ($input:expr, $expected:expr,) => {
            check!($input, $expected)
        };
        ($input:expr, $expected:expr) => {{
            let mut text = str!($input);
            normalize(&mut text);
            assert_eq!(text, $expected, "Normalized text doesn't match expected");
        }};
    }

    check!("", "");
    check!("Big Cheese Horace", "big-cheese-horace");
    check!("bottom--Text", "bottom-text");
    check!("Tufto's Proposal", "tufto-s-proposal");
    check!(" - Test - ", "test");
    check!("--TEST--", "test");
    check!("-test-", "test");
    check!(":test", "test");
    check!("test:", "test");
    check!(":test:", "test");
    check!("/Some Page", "some-page");
    check!("some/Page", "some-page");
    check!("some,Page", "some-page");
    check!("End of Death Hub", "end-of-death-hub");
    check!("$100 is a lot of money", "100-is-a-lot-of-money");
    check!("snake_case", "snake-case");
    check!("long__snake__case", "long-snake-case");
    check!("_template", "_template");
    check!("_template_", "_template");
    check!("__template", "_template");
    check!("__template_", "_template");
    check!("template_", "template");
    check!("template__", "template");
    check!("_Template", "_template");
    check!("_Template_", "_template");
    check!("__Template", "_template");
    check!("__Template_", "_template");
    check!("Template_", "template");
    check!("Template__", "template");
    check!(" <[ TEST ]> ", "test");
    check!("ÄÀ-áö ðñæ_þß*řƒŦ", "aa-aoe-dnae-tss-rft");
    check!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!", "");
    check!("Component:image block", "component:image-block");
    check!("fragment:scp-4447-2", "fragment:scp-4447-2");
    check!("fragment::scp-4447-2", "fragment:scp-4447-2");
    check!("FRAGMENT:SCP-4447 (2)", "fragment:scp-4447-2");
    check!("fragment:_template", "fragment:_template");
    check!("fragment:__template", "fragment:_template");
    check!("fragment:_template_", "fragment:_template");
    check!("fragment::_template", "fragment:_template");
    check!(
        "protected:fragment:_template",
        "protected:fragment:_template",
    );
    check!(
        "protected:fragment:__template",
        "protected:fragment:_template",
    );
    check!(
        "protected:fragment:_template_",
        "protected:fragment:_template",
    );
    check!(
        "protected:fragment::_template",
        "protected:fragment:_template",
    );
    check!(
        "protected::fragment:_template",
        "protected:fragment:_template",
    );
    check!(
        "protected::fragment::_template",
        "protected:fragment:_template",
    );
}
