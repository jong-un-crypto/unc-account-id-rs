# unc-account-id

This crate provides a type for representing a syntactically valid, unique account identifier on the [Utility](https://unc.org) network, according to the [Utility Account ID](https://docs.unc.org/concepts/basics/account#account-id-rules) rules.

[![crates.io](https://img.shields.io/crates/v/unc-account-id?label=latest)](https://crates.io/crates/unc-account-id)
[![Documentation](https://docs.rs/unc-account-id/badge.svg)](https://docs.rs/unc-account-id)

## Usage

```rust
use unc_account_id::AccountId;

let alice: AccountId = "alice.unc".parse()?;

assert!("ƒelicia.unc".parse::<AccountId>().is_err()); // (ƒ is not f)
```

See the [docs](https://docs.rs/unc-account-id) for more information.

## Minimum Supported Rust Version (MSRV)

1.65

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
