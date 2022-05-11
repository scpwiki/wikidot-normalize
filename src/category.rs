/*
 * category.rs
 *
 * wikidot-normalize - Library to provide Wikidot-compatible normalization.
 * Copyright (c) 2019-2022 Ammon Smith
 *
 * wikidot-normalize is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

pub fn merge_multi_categories(text: &mut String) {
    let mut indices = vec![];
    let mut first = true;

    // Find all colons except the last
    for (idx, ch) in text.char_indices().rev() {
        if ch != ':' {
            continue;
        }

        if first {
            first = false;
            continue;
        }

        indices.push(idx);
    }

    // Replace all colons with dashes
    for idx in indices {
        text.replace_range(idx..idx + 1, "-");
    }
}

#[test]
fn test_multi_category() {
    macro_rules! check {
        ($input:expr, $expected:expr $(,)?) => {{
            let mut text = str!($input);
            merge_multi_categories(&mut text);
            assert_eq!(
                text, $expected,
                "Merged multiple categories doesn't match expected",
            );
        }};
    }

    check!("", "");
    check!("alpha", "alpha");
    check!("alpha:beta", "alpha:beta");
    check!("alpha:beta:gamma", "alpha-beta:gamma");
    check!("alpha:beta:gamma:delta", "alpha-beta-gamma:delta");
    check!(
        "alpha:beta:gamma:delta:epsilon",
        "alpha-beta-gamma-delta:epsilon",
    );
}
