## wikidot-normalize

<p>
  <a href="https://github.com/scpwiki/wikidot-normalize/actions?query=workflow%3A%22Rust+CI%22">
    <img src="https://github.com/scpwiki/wikidot-normalize/workflows/Rust%20CI/badge.svg"
         alt="Rust CI badge">
  </a>

  <a href="https://docs.rs/wikidot-normalize">
    <img src="https://docs.rs/wikidot-normalize/badge.svg"
         alt="docs.rs link">
  </a>
</p>

Simple library to provide Wikidot-compatible string normalization. It is a Rust port of the functionality in [`WDStringUtils::toUnixName`](https://github.com/scpwiki/wikijump/blob/develop/web/php/Utils/WDStringUtils.php).

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

This library is getting close to finalization with a `v1.0.0` release.

Available under the terms of the MIT License. See [LICENSE.md](LICENSE).

### Compilation
This library targets the latest stable Rust. At time of writing, that is 1.55.0

```sh
$ cargo build --release
```

### Testing
```sh
$ cargo test
```

Add `-- --nocapture` to the end if you want to see test output.
