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
    for (idx, ch) in text.char_indices() {
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
        text.replace_range(idx..idx + 1, "-");
    }
}

#[test]
fn test_replace_underscores() {
    macro_rules! check {
        ($input:expr, $expected:expr,) => {
            check!($input, $expected)
        };
        ($input:expr, $expected:expr) => {{
            let mut text = str!($input);
            replace_underscores(&mut text);

            assert_eq!(
                &text, $expected,
                "Underscore replacement didn't match expected (input: '{}')",
                $input,
            );
        }};
    }

    check!("", "");
    check!("a", "a");
    check!("_a", "_a");
    check!("a_", "a-");
    check!("page-name", "page-name");
    check!("_template", "_template");
    check!("__template", "_-template");
    check!("_template_", "_template-");
    check!("_special_page", "_special-page");
    check!("_special__page", "_special--page");
    check!("fragment:page-name", "fragment:page-name");
    check!("fragment:_template", "fragment:_template");
    check!("fragment:__template", "fragment:_-template");
    check!("fragment:_template_", "fragment:_template-");
    check!("fragment:_special_page", "fragment:_special-page");
    check!("fragment:_special__page", "fragment:_special--page");
    check!("_default:page-name", "_default:page-name");
    check!("_default:_template", "_default:_template");
    check!("_default:__template", "_default:_-template");
    check!("_default:_template_", "_default:_template-");
    check!("_default:_special_page", "_default:_special-page");
    check!("_default:_special__page", "_default:_special--page");
    check!(
        "protected:fragment:page-name",
        "protected:fragment:page-name",
    );
    check!(
        "protected:fragment:_template",
        "protected:fragment:_template",
    );
    check!(
        "protected:fragment:__template",
        "protected:fragment:_-template",
    );
    check!(
        "protected:fragment:_template_",
        "protected:fragment:_template-",
    );
    check!(
        "protected:fragment:_special_page",
        "protected:fragment:_special-page",
    );
    check!(
        "protected:fragment:_special__page",
        "protected:fragment:_special--page",
    );
}
