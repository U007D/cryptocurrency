use crate::{error::InternalError, tx::TxIdx, Error, Result, Signature, TxHash};
use bincode::serialize;
use serde::Serialize;

#[allow(bare_trait_objects)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum InputTx {
    Genesis,
    Unsigned {
        /// Used output's index in the previous tx
        output_idx: TxIdx,
        /// Hash of the tx whose output is being used
        prev_tx_hash: TxHash,
    },
    Signed {
        /// Used output's index in the previous tx
        output_idx: TxIdx,
        /// Hash of the tx whose output is being used
        prev_tx_hash: TxHash,
        /// The signature produced to check validity
        signature: Signature,
    },
}

impl InputTx {
    #[must_use]
    pub fn new(prev_tx_hash: TxHash, output_idx: TxIdx) -> Self {
        Self::Unsigned {
            output_idx,
            prev_tx_hash,
        }
    }

    pub fn sign(self, signature: Signature) -> Result<Self> {
        Ok(match self {
            Self::Unsigned {
                output_idx,
                prev_tx_hash,
            }
            | Self::Signed {
                output_idx,
                prev_tx_hash,
                signature: _,
            } => Self::Signed {
                output_idx,
                prev_tx_hash,
                signature,
            },
            Self::Genesis => Err(Error::CannotSignGenesisBlock)?,
        })
    }

    pub fn unsign(self) -> Result<Self> {
        Ok(match self {
            Self::Unsigned {
                output_idx,
                prev_tx_hash,
            }
            | Self::Signed {
                output_idx,
                prev_tx_hash,
                signature: _,
            } => Self::Unsigned {
                output_idx,
                prev_tx_hash,
            },
            Self::Genesis => Err(Error::CannotUnsignGenesisBlock)?,
        })
    }

    #[must_use]
    pub fn as_bytes(&self) -> Vec<u8> {
        serialize(self).unwrap_or_else(|err| panic!(InternalError::Serialization(err)))
    }
}
