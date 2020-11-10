/*
 * test.rs
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

use crate::prelude::*;

#[test]
fn test_normalize() {
    macro_rules! check {
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
    check!("-test-", "test");
    check!("End of Death Hub", "end-of-death-hub");
    check!("$100 is a lot of money", "100-is-a-lot-of-money");
    check!("snake_case", "snake-case");
    check!("long__snake__case", "long-snake-case");
    check!("_template", "_template");
    check!("__template", "_template");
    check!("template_", "template");
    check!("template__", "template");
    check!(" <[ TEST ]> ", "test");
    check!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!", "");
    check!("Component:image block", "component:image-block");
    check!("fragment:scp-4447-2", "fragment:scp-4447-2");
    check!("fragment::scp-4447-2", "fragment:scp-4447-2");
}

#[test]
fn test_is_normal() {
    macro_rules! check {
        ($expected:expr, $input:expr) => {{
            assert_eq!(
                is_normal($input),
                $expected,
                "Normalization test failed: {}",
                $input,
            );
        }};
    }

    check!(true, "");
    check!(true, "big-cheese-horace");
    check!(false, "Big Cheese Horace");
    check!(true, "bottom-text");
    check!(false, "bottom-Text");
    check!(false, "-test-");
    check!(true, "scp-1000");
    check!(true, "end-of-death-hub");
    check!(false, "End of Death Hub");
    check!(false, "$200 please");
    check!(true, "kebab-case");
    check!(false, "snake_case");
    check!(true, "_template");
    check!(false, "__template");
    check!(false, "<[ TEST ]>");
    check!(false, " <[ TEST ]> ");
    check!(false, "!!!!!!!!!!!!");
    check!(false, "Component:image-block");
    check!(true, "component:image-block");
    check!(false, "fragment::scp-4447-2");
}
