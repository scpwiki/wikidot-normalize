## wikidot-normalize
[![Build Status](https://travis-ci.org/Nu-SCPTheme/wikidot-normalize.svg?branch=master)](https://travis-ci.org/Nu-SCPTheme/wikidot-normalize)

Simple library to provide Wikidot-compatible string normalization. It attempts to be a Rust port of the functionality in [`WDStringUtils::toUnixName`](https://github.com/scpwiki/wikijump/blob/master/php/utils/WDStringUtils.php).

Wikidot normal form is used in the site's page names. Essentially it ensures the following:

* All ASCII is lowercase.
* All characters outside of `:`, `a-z`, `0-9`, or `-` are replaced with dashes.
* Underscores are only permitted as the first character.
* Any leading or trailing dashes are removed.
* Any set of multiple dashes are replaced with a single dash.
* Any set of multiple colons are replaced with a single colon.

**Examples:**

* `"Big Cheese Horace"` **->** `"big-cheese-horace"`
* `"bottom--Text"` **->** `"bottom-text"`
* `"Tufto's Proposal"` **->** `"tufto-s-proposal"`
* `"-test-"` **->** `"test"`

Available under the terms of the MIT License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.47.0

```sh
$ cargo build --release
```

### Testing
```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.
