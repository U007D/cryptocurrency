mod address;
mod input_tx;
mod non_empty_ext;
mod operation;
mod output_tx;
mod signature;
mod tx_hash;
mod tx_idx;

use crate::{Error, Result, Utxo};
pub use address::Address;
pub use input_tx::InputTx;
use non_empty_ext::NonEmptyExt;
use nonempty::NonEmpty as NonEmptyVec;
pub use operation::Operation;
pub use output_tx::OutputTx;
use rust_decimal::Decimal;
use sha2::{Digest, Sha256};
pub use signature::Signature;
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TxBuilder {
    input_txs: Vec<InputTx>,
    output_txs: Vec<OutputTx>,
}

impl TxBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_input(&mut self, prev_tx_hash: TxHash, output_idx: TxIdx) -> &mut Self {
        self.input_txs.push(InputTx::new(prev_tx_hash, output_idx));
        self
    }

    pub fn add_output(&mut self, value: Decimal, address: Address) -> &mut Self {
        self.output_txs.push(OutputTx::new(value, address));
        self
    }

    // TODO: This is about as expensive as possible; refactor using `HashMap` and less (no?) copying
    fn remove_input(&mut self, utxo: &Utxo) -> Result<&mut Self> {
        self.input_txs = self
            .input_txs
            .iter()
            .filter(|el| utxo != *el)
            .cloned()
            .collect();
        Ok(self)
    }

    fn raw_tx_unsigned(&self, idx: usize) -> Option<Vec<u8>> {
        self.input_txs.get(idx).map(|tx| {
            match tx {
                InputTx::Signed {
                    output_idx: _,
                    prev_tx_hash: _,
                    signature: _,
                } => tx
                    .clone()
                    .unsign()
                    .unwrap_or_else(|err| unreachable!(err))
                    .as_bytes(),
                t => t.as_bytes(),
            }
            .into_iter()
            .chain(
                self.output_txs
                    .iter()
                    .flat_map(|tx| tx.as_bytes().into_iter()),
            )
            .collect()
        })
    }

    pub fn add_signature(&mut self, signature: Signature, idx: TxIdx) -> Result<&mut Self> {
        self.input_txs
            .get_mut(usize::from(idx))
            .ok_or_else(|| Error::TxnIdxOutOfBounds(idx))
            .map(|el| el.clone().sign(signature).map(|signed| *el = signed))??;
        Ok(self)
    }

    fn raw_tx(&self) -> Vec<u8> {
        self.input_txs
            .iter()
            .flat_map(|tx| tx.as_bytes().into_iter())
            .chain(
                self.output_txs
                    .iter()
                    .flat_map(|tx| tx.as_bytes().into_iter()),
            )
            .collect()
    }

    pub fn build(self) -> Result<Tx> {
        Ok(Tx {
            hash: self.hash_tx(),
            inputs: NonEmptyVec::try_from(self.input_txs)
                .ok_or_else(|| Error::CannotBuildTxWithoutInputTxs)?,
            outputs: NonEmptyVec::try_from(self.output_txs)
                .ok_or_else(|| Error::CannotBuildTxWithoutOutputTxs)?,
        })
    }

    fn hash_tx(&self) -> TxHash {
        let mut hasher = Sha256::new();
        hasher.input(&self.raw_tx());
        TxHash(hasher.result().as_slice().to_vec())
    }

    pub const fn input_txs(&self) -> &Vec<InputTx> {
        &self.input_txs
    }

    pub const fn output_txs(&self) -> &Vec<OutputTx> {
        &self.output_txs
    }

    pub fn input_tx(&self, idx: TxIdx) -> Option<&InputTx> {
        self.input_txs.get(usize::from(idx))
    }

    pub fn output_tx(&self, idx: TxIdx) -> Option<&OutputTx> {
        self.output_txs.get(usize::from(idx))
    }

    pub fn n_input_txs(&self) -> usize {
        self.input_txs.len()
    }

    pub fn n_output_txs(&self) -> usize {
        self.output_txs.len()
    }
}
