use crate::{consts, tx::Address, Error, Result, Signature};
use serde::{Serialize, Serializer};
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug, Eq)]
pub struct PublicKey(ed25519_dalek::PublicKey);

impl PublicKey {
    pub fn verify(
        &self,
        message: &[u8],
        signature: &Signature,
    ) -> Result<(), ed25519_dalek::errors::SignatureError> {
        self.0.verify::<sha2::Sha256>(message, &signature.0)
    }
}

impl Address for PublicKey {
    type Error = Error;

    fn from_slice(slice: impl AsRef<[u8]>) -> Result<Self, Self::Error> {
        Ok(Self(ed25519_dalek::PublicKey::from_bytes(slice.as_ref())?))
    }

    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl Hash for PublicKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash_slice(self.0.as_bytes(), state)
    }
}

impl PartialEq for PublicKey {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.as_bytes().eq(rhs.0.as_bytes())
    }
}

impl Serialize for PublicKey {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct(
            consts::ED25519_DALEK_PUBLIC_KEY_TYPE_NAME,
            self.0.as_bytes(),
        )
    }
}
