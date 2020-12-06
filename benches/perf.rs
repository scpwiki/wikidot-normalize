/*
 * benches/perf.rs
 *
 * ftml - Library to parse Wikidot code
 * Copyright (C) 2019-2020 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

#![allow(soft_unstable)]

//! Transient file to occassionally utilize to benchmark the library's performance.
//!
//! On a separate branch because this Rust feature requires nightly.

#[macro_use]
extern crate bencher;
extern crate wikidot_normalize;

#[macro_use]
extern crate str_macro;

use bencher::Bencher;

fn normalize(bench: &mut Bencher) {
    const INPUTS: [&str; 48] = [
        "",
        "Big Cheese Horace",
        "bottom--Text",
        "Tufto's Proposal",
        " - Test - ",
        "--TEST--",
        "-test-",
        ":test",
        "test:",
        ":test:",
        "/Some Page",
        "some/Page",
        "some,Page",
        "End of Death Hub",
        "$100 is a lot of money",
        "snake_case",
        "long__snake__case",
        "_template",
        "_template_",
        "__template",
        "__template_",
        "template_",
        "template__",
        "_Template",
        "_Template_",
        "__Template",
        "__Template_",
        "Template_",
        "Template__",
        " <[ TEST ]> ",
        "ÄÀ-áö ðñæ_þß*řƒŦ",
        "!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!",
        "Component:image block",
        "fragment:scp-4447-2",
        "fragment::scp-4447-2",
        "FRAGMENT:SCP-4447 (2)",
        "protected_:fragment_:page",
        "protected:_fragment_:page",
        "fragment:_template",
        "fragment:__template",
        "fragment:_template_",
        "fragment::_template",
        "protected:fragment:_template",
        "protected:fragment:__template",
        "protected:fragment:_template_",
        "protected:fragment::_template",
        "protected::fragment:_template",
        "protected::fragment::_template",
    ];

    bench.iter(|| {
        for input in &INPUTS[..] {
            let mut text = str!(input);

            wikidot_normalize::normalize(&mut text);
        }
    });
}

benchmark_group!(benches, normalize);
benchmark_main!(benches);
