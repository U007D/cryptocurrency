use crate::tx::{InputTx, TxHash, TxIdx};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Utxo {
    /// Index of the corresponding output in said tx
    tx_idx: TxIdx,
    /// Hash of the tx from which this `Utxo` originates
    tx_hash: TxHash,
}

impl Utxo {
    /// Creates a new Utxo corresponding to the output with the index `txIndex` in the tx
    /// whose hash is `txHash`
    #[must_use]
    pub const fn new(tx_hash: TxHash, tx_idx: TxIdx) -> Self {
        Self { tx_idx, tx_hash }
    }

    /// Return the tx hash of this `Utxo`
    #[must_use]
    pub const fn tx_hash(&self) -> &TxHash {
        &self.tx_hash
    }

    /// Return the index of this `Utxo`
    #[must_use]
    pub const fn tx_idx(&self) -> &TxIdx {
        &self.tx_idx
    }
}

impl PartialEq<InputTx> for Utxo {
    fn eq(&self, rhs: &InputTx) -> bool {
        match rhs {
            InputTx::Unsigned {
                output_idx,
                prev_tx_hash,
            }
            | InputTx::Signed {
                output_idx,
                prev_tx_hash,
                signature: _,
            } => self.tx_idx == *output_idx && self.tx_hash == *prev_tx_hash,
            _ => false,
        }
    }
}
