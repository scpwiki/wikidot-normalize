/*
 * unicode.rs
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

use std::mem;
use unicode_normalization::UnicodeNormalization;

pub fn normalize_nfkc(text: &mut String) {
    let mut normalized = text.nfkc().collect();

    mem::swap(text, &mut normalized);
}
