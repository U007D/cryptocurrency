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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tx {
    /// Hash of the tx, its unique ID
    hash: TxHash,
    input_txs: NonEmptyVec<InputTx>,
    output_txs: NonEmptyVec<OutputTx>,
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

    pub fn input_txs(&self) -> impl Iterator<Item = &InputTx> {
        self.input_txs.iter()
    }

    pub fn output_txs(&self) -> impl Iterator<Item = &OutputTx> {
        self.output_txs.iter()
    }

    #[must_use]
    pub fn input_tx(&self, idx: TxIdx) -> Option<&InputTx> {
        self.input_txs.get(usize::from(idx))
    }

    #[must_use]
    pub fn output_tx(&self, idx: TxIdx) -> Option<&OutputTx> {
        self.output_txs.get(usize::from(idx))
    }

    #[must_use]
    pub fn n_input_txs(&self) -> usize {
        self.input_txs.len()
    }

    #[must_use]
    pub fn n_output_txs(&self) -> usize {
        self.output_txs.len()
    }
}
