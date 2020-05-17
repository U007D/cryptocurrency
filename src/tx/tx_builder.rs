use super::non_empty_ext::NonEmptyExt;
pub use crate::{
    tx::{Address, InputTx, OutputTx, PublicKey, SecretKey, Tx},
    TxHash, TxIdx,
};
use crate::{Error, Result, Signature};
use nonempty::NonEmpty as NonEmptyVec;
use rust_decimal::Decimal;
use sha2::{Digest, Sha256};

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

    pub fn add_output(&mut self, value: Decimal, address: PublicKey) -> &mut Self {
        self.output_txs.push(OutputTx::new(value, address));
        self
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
            input_txs: NonEmptyVec::try_from(self.input_txs)
                .ok_or_else(|| Error::CannotBuildTxWithoutInputTxs)?,
            output_txs: NonEmptyVec::try_from(self.output_txs)
                .ok_or_else(|| Error::CannotBuildTxWithoutOutputTxs)?,
        })
    }

    fn hash_tx(&self) -> TxHash {
        let mut hasher = Sha256::new();
        hasher.input(&self.raw_tx());
        TxHash(hasher.result().as_slice().to_vec())
    }
}
