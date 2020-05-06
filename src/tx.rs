mod address;
mod input_tx;
mod non_empty_ext;
mod operation;
mod output_tx;
mod tx_builder;
mod tx_hash;
mod tx_idx;

pub use address::{Address, PublicKey, SecretKey};
pub use input_tx::InputTx;
use nonempty::NonEmpty as NonEmptyVec;
pub use operation::Operation;
pub use output_tx::OutputTx;
pub use tx_builder::TxBuilder;
pub use tx_hash::TxHash;
pub use tx_idx::TxIdx;

pub struct Tx {
    /// Hash of the tx, its unique ID
    hash: TxHash,
    inputs: NonEmptyVec<InputTx>,
    outputs: NonEmptyVec<OutputTx>,
}

#[allow(clippy::new_ret_no_self)]
impl Tx {
    #[must_use]
    pub fn new() -> TxBuilder {
        TxBuilder::new()
    }

    #[must_use]
    pub const fn hash(&self) -> &TxHash {
        &self.hash
    }
}
