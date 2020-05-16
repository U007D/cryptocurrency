use crate::error::PanicError;
use crate::tx::PublicKey;
use bincode::serialize;
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct OutputTx {
    /// Value in BitCoins of the output
    value: Decimal,
    /// The address (public key) of the recipient
    address: PublicKey,
}

impl OutputTx {
    #[must_use]
    pub const fn new(value: Decimal, address: PublicKey) -> Self {
        Self { value, address }
    }

    #[must_use]
    pub const fn address(&self) -> &PublicKey {
        &self.address
    }

    #[must_use]
    pub fn as_bytes(&self) -> Vec<u8> {
        serialize(self).unwrap_or_else(|err| panic!(PanicError::Serialization(err)))
    }

    #[must_use]
    pub const fn value(&self) -> Decimal {
        self.value
    }
}
