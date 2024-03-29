/*
 * unicode.rs
 *
 * wikidot-normalize - Library to provide Wikidot-compatible normalization.
 * Copyright (c) 2019-2023 Emmie Maeda
 *
 * wikidot-normalize is available free of charge under the terms of the MIT
 * License. You are free to redistribute and/or modify it under those
 * terms. It is distributed in the hopes that it will be useful, but
 * WITHOUT ANY WARRANTY. See the LICENSE file for more details.
 *
 */

use std::mem;
use unicode_normalization::UnicodeNormalization;

pub fn normalize_nfkc(text: &mut String) {
    let mut normalized = text.nfkc().collect();

    mem::swap(text, &mut normalized);
}

pub fn casefold(text: &mut String) {
    let mut folded = String::new();

    for ch in text.chars() {
        folded.extend(ch.to_lowercase());
    }

    mem::swap(text, &mut folded);
}
