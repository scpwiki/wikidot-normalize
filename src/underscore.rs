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
            // If it's not the leading underscore, or after a category,
            // push an index to replace it
            if idx > 0 && !prev_colon {
                matches.push(idx);
            }
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
    macro_rules! test {
        ($input:expr, $expected:expr) => {{
            let mut text = str!($input);
            replace_underscores(&mut text);

            assert_eq!(
                &text,
                $expected,
                "Underscore replacement didn't match expected (input: '{}')",
                $input,
            );
        }};
    }

    test!("", "");
    test!("page-name", "page-name");
    test!("_template", "_template");
    test!("__template", "_-template");
    test!("_template_", "_template-");
    test!("_special_page", "_special-page");
    test!("_special__page", "_special--page");
    test!("fragment:page-name", "fragment:page-name");
    test!("fragment:_template", "fragment:_template");
    test!("fragment:__template", "fragment:_-template");
    test!("fragment:_template_", "fragment:_template-");
    test!("fragment:_special_page", "fragment:_special-page");
    test!("fragment:_special__page", "fragment:_special--page");
    test!("protected:fragment:page-name", "protected:fragment:page-name");
    test!("protected:fragment:_template", "protected:fragment:_template");
    test!("protected:fragment:__template", "protected:fragment:_-template");
    test!("protected:fragment:_template_", "protected:fragment:_template-");
    test!("protected:fragment:_special_page", "protected:fragment:_special-page");
    test!("protected:fragment:_special__page", "protected:fragment:_special--page");
}
