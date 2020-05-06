use crate::tx::Address;
use crate::{
    consts,
    tx::{PublicKey, SecretKey},
};
use rand::{CryptoRng, Rng};
use serde::{Serialize, Serializer};
use sha2::Sha512;
use std::hash::{Hash, Hasher};

pub struct KeyPair(ed25519_dalek::Keypair);

impl KeyPair {
    pub fn new<R: CryptoRng + Rng>(csprng: &mut R) -> Self {
        Self(ed25519_dalek::Keypair::generate::<Sha512, R>(csprng))
    }

    #[must_use]
    pub fn public_key(&self) -> PublicKey {
        PublicKey::from_slice(self.0.public.as_bytes()).unwrap_or_else(|err| unreachable!(err))
    }

    #[must_use]
    pub fn secret_key(&self) -> SecretKey {
        SecretKey::from_slice(self.0.secret.as_bytes()).unwrap_or_else(|err| unreachable!(err))
    }
}

impl Hash for KeyPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash_slice(&self.0.to_bytes(), state)
    }
}

impl Serialize for KeyPair {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct(
            consts::ED25519_DALEK_KEY_PAIR_TYPE_NAME,
            self.0.to_bytes().as_ref(),
        )
    }
}
