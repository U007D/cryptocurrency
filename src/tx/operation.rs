use crate::tx::tx_hash::TxHash;

/// Define the operations that can be performed on a tx
#[derive(Debug)]
pub enum Operation {
    Sign(TxHash),
    Unsign,
}
