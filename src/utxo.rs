use crate::tx::{InputTx, TxHash, TxIdx};
use serde::Serialize;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Utxo {
    /// Hash of the tx from which this `Utxo` originates
    tx_hash: TxHash,
    /// Index of the corresponding output in said tx
    tx_idx: TxIdx,
}

impl Utxo {
    /// Creates a new Utxo corresponding to the output with the index `txIndex` in the tx
    /// whose hash is `txHash`
    #[must_use]
    pub const fn new(tx_hash: TxHash, tx_idx: TxIdx) -> Self {
        Self { tx_idx, tx_hash }
    }

    #[must_use]
    pub fn as_vec(&self) -> Vec<u8> {
        [self.tx_hash().as_bytes(), &self.tx_idx().as_arr()].concat()
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
            InputTx::Unsigned { output_utxo }
            | InputTx::Signed {
                output_utxo,
                signature: _,
            } => self.tx_idx() == output_utxo.tx_idx() && self.tx_hash() == output_utxo.tx_hash(),
            _ => false,
        }
    }
}
