// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    rust_2018_idioms,
    clippy::unused_self
)]
#![allow(
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::try_err
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, clippy::missing_errors_doc, warnings)]
//#![deny(warnings)]

mod consts;
mod crypto;
mod error;
mod key_pair;
mod signature;
mod tx;
mod tx_handler;
mod utxo;
mod utxo_pool;
pub use {
    crypto::verify_signature,
    error::Error,
    key_pair::KeyPair,
    signature::Signature,
    tx::{InputTx, Operation, OutputTx, PublicKey, SecretKey, Tx, TxHash, TxIdx},
    tx_handler::TxHandler,
    utxo::Utxo,
};
pub type Result<T, E = Error> = std::result::Result<T, E>;
