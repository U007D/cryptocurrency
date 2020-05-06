use crate::tx::PublicKey;
use crate::Signature;

/// return true if `signature` is a valid digital signature of `message` under the
///         key `pub_key`. Internally, this uses RSA signature, but the student does not
///         have to deal with any of the implementation details of the specific signature
///         algorithm
#[must_use]
pub const fn verify_signature(pub_key: &PublicKey, message: &[u8], signature: &Signature) -> bool {
    // TODO: Implement.  Check `InputTx::Signed`?  Or unsigned portion of any message?
    false
}
