use crate::tx::TxIdx;
use crate::{consts::msg, Utxo};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}: {:?}", msg::ERR_ADD_DUPLICATE_UTXO_ATTEMPTED, 0)]
    AddDuplicateUtxoAttempted(Utxo),
    #[error("{}: {:?}", msg::ERR_REMOVE_UTXO_NOT_FOUND, 0)]
    UtxoNotFound(Utxo),
    #[error("{}.", msg::ERR_CANNOT_SIGN_GENESIS)]
    CannotSignGenesisBlock,
    #[error("{}.", msg::ERR_CANNOT_UNSIGN_GENESIS)]
    CannotUnsignGenesisBlock,
    #[error("{}.", msg::ERR_CANNOT_BUILD_TX_WO_INPUT_TXS)]
    CannotBuildTxWithoutInputTxs,
    #[error("{}.", msg::ERR_CANNOT_BUILD_TX_WO_OUTPUT_TXS)]
    CannotBuildTxWithoutOutputTxs,
    #[error("{}.", msg::ERR_TX_IDX_OUT_OF_BOUNDS)]
    TxnIdxOutOfBounds(TxIdx),
}

#[derive(Debug, Error)]
pub enum InternalError {
    #[error("{}: {}", msg::ERR_INTERNAL_SERIALIZATION, 0)]
    Serialization(#[from] bincode::Error),
}
