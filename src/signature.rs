use crate::consts;
use serde::{Serialize, Serializer};
use std::{
    fmt::Formatter,
    fmt::{Debug, Display, Result as FmtResult},
    hash::{Hash, Hasher},
};

#[derive(Clone, Debug, Eq)]
pub struct Signature(pub(crate) ed25519_dalek::Signature);

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash_slice(&self.0.to_bytes(), state)
    }
}

impl PartialEq for Signature {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.to_bytes().iter().eq(rhs.0.to_bytes().iter())
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct(
            consts::ED25519_DALEK_SIGNATURE_TYPE_NAME,
            self.0.to_bytes().as_ref(),
        )
    }
}
