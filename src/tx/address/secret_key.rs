use crate::{consts, tx::Address, Error, Result};
use serde::{Serialize, Serializer};
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct SecretKey(ed25519_dalek::SecretKey);

impl Address for SecretKey {
    type Error = Error;

    fn from_slice(slice: impl AsRef<[u8]>) -> Result<Self, Self::Error> {
        Ok(Self(ed25519_dalek::SecretKey::from_bytes(slice.as_ref())?))
    }

    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Display for SecretKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl Eq for SecretKey {}
impl Hash for SecretKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_bytes().hash(state)
    }
}

impl PartialEq for SecretKey {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.as_bytes().eq(rhs.0.as_bytes())
    }
}

impl Serialize for SecretKey {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct(
            consts::ED25519_DALEK_SECRET_KEY_TYPE_NAME,
            self.0.as_bytes(),
        )
    }
}
