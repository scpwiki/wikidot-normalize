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
    check!("snake_case", "snake_case");
    check!("long__snake__case", "long__snake__case");
    check!(" <[ TEST ]> ", "test");
    check!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!", "");

    check!("/", "/");
    check!("/scp-1000/", "/scp-1000/");
    check!("/SCP 4447/ofFSEt/2", "/scp-4447/offset/2");
    check!("page/discuss", "page/discuss");
    check!("/-test-/", "/test/");
    check!("/Tufto's Proposal---", "/tufto-s-proposal");
    check!("page/-", "page/");
    check!("/ page /-yeah-/ thing ", "/page/yeah/thing");

    check!("/SCP%20xxxx", "/scp-20xxxx");
    check!("/scp%20xxxx/", "/scp-20xxxx/");
    check!("%20scp%20%20xxxx", "20scp-20-20xxxx");
}

#[test]
fn test_normalize_decode() {
    macro_rules! check {
        ($input:expr, $expected:expr) => {{
            let mut text = str!($input);
            normalize_decode(&mut text);
            assert_eq!(text, $expected, "Normalized text doesn't match expected");
        }};
    }

    check!("/SCP%20xxxx", "/scp-xxxx");
    check!("/scp%20xxxx/", "/scp-xxxx/");
    check!("%20scp%20%20xxxx", "scp-xxxx");
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
    check!(true, "snake_case");
    check!(true, "kebab-case");
    check!(false, "<[ TEST ]>");
    check!(false, " <[ TEST ]> ");
    check!(false, "!!!!!!!!!!!!");

    check!(true, "/");
    check!(true, "/scp-1000/");
    check!(false, "/SCP-1000/");
    check!(true, "/scp-4447/offset/2");
    check!(false, "/SCP 4447/ofFSEt/2");
    check!(true, "page/discuss");
    check!(false, "/-test-/");
    check!(true, "/test/");
    check!(false, "/Tufto's Proposal---");
    check!(false, "/ page /-yeah-/ thing");
    check!(false, "/ page /-yeah-/ ");
    check!(false, "/ page /-yeah-");
    check!(false, "/ page /-");
    check!(false, "/ page");

    check!(false, "/scp xxxx");
    check!(false, "/scp%20xxxx");
    check!(false, "/SCP%20xxxx");
    check!(true, "/scp-xxxx");
}
