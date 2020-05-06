mod public_key;
mod secret_key;
pub use public_key::PublicKey;
pub use secret_key::SecretKey;

use std::error::Error as StdError;

pub trait Address {
    type Error: StdError;
    fn from_slice(slice: impl AsRef<[u8]>) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn as_bytes(&self) -> &[u8];
}
