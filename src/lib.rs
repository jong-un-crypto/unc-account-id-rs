//! This crate provides a type for representing a syntactically valid, unique account identifier on the [Utility](https://unc.org) network.
//!
//! ## Account ID Rules
//!
//! - Minimum length is `2`
//! - Maximum length is `64`
//! - An **Account ID** consists of **Account ID parts** separated by `.`, example:
//!   - `root` ✔
//!   - `alice.unc` ✔
//!   - `app.stage.testnet` ✔
//! - Must not start or end with separators (`_`, `-` or `.`):
//!   - `_alice.` ✗
//!   - `.bob.unc-` ✗
//! - Each part of the **Account ID** consists of lowercase alphanumeric symbols separated either by `_` or `-`, example:
//!   - `ƒelicia.unc` ✗ (`ƒ` is not `f`)
//!   - `1_4m_n0t-al1c3.unc` ✔
//! - Separators are not permitted to immediately follow each other, example:
//!   - `alice..unc` ✗
//!   - `not-_alice.unc` ✗
//! - An **Account ID** that is 64 characters long and consists of lowercase hex characters is a specific **implicit account ID**
//!
//! Learn more here: <https://docs.unc.org/docs/concepts/account#account-id-rules>
//!
//! Also see [Error kind precedence](AccountId#error-kind-precedence).
//!
//! ## Usage
//!
//! ```
//! use unc_account_id::{AccountIdRef, AccountId};
//!
//! const ALICE: &AccountIdRef = AccountIdRef::new_or_panic("alice.unc");
//!
//! let alice: AccountId = "alice.unc".parse().unwrap();
//!
//! assert!("ƒelicia.unc".parse::<AccountId>().is_err()); // (ƒ is not f)
//! ```

mod errors;

mod account_id;
mod account_id_ref;
#[cfg(feature = "borsh")]
mod borsh;
#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod test_data;
mod validation;

pub use account_id::AccountId;
pub use account_id_ref::{AccountIdRef, AccountType};
pub use errors::{ParseAccountError, ParseErrorKind};
