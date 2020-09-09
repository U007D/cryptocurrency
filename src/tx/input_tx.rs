use crate::{error::PanicError, tx::TxIdx, Error, Result, Signature, TxHash, Utxo};
use bincode::serialize;
use serde::Serialize;

#[allow(bare_trait_objects)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum InputTx {
    Genesis,
    Unsigned {
        /// Used output's index in the previous tx
        /// Hash of the tx whose output is being used
        output_utxo: Utxo,
    },
    Signed {
        /// Used output's index in the previous tx
        /// Hash of the tx whose output is being used
        output_utxo: Utxo,
        /// The signature produced to check validity
        signature: Signature,
    },
}

impl InputTx {
    #[must_use]
    pub const fn new(prev_tx_hash: TxHash, output_idx: TxIdx) -> Self {
        Self::Unsigned {
            output_utxo: Utxo::new(prev_tx_hash, output_idx),
        }
    }

    pub fn sign(self, signature: Signature) -> Result<Self> {
        Ok(match self {
            Self::Unsigned { output_utxo }
            | Self::Signed {
                output_utxo,
                signature: _,
            } => Self::Signed {
                output_utxo,
                signature,
            },
            Self::Genesis => Err(Error::CannotSignGenesisBlock)?,
        })
    }

    pub fn unsign(self) -> Result<Self> {
        Ok(match self {
            Self::Unsigned { output_utxo }
            | Self::Signed {
                output_utxo,
                signature: _,
            } => Self::Unsigned { output_utxo },
            Self::Genesis => Err(Error::CannotUnsignGenesisBlock)?,
        })
    }

    #[must_use]
    pub fn as_bytes(&self) -> Vec<u8> {
        serialize(self).unwrap_or_else(|err| panic!(PanicError::Serialization(err)))
    }
}
