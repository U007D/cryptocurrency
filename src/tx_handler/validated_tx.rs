use crate::Tx;
use std::ops::Deref;

/// `add_validated_output_txs_to_utxo_pool()` and `remove_validated_input_txs_from_utxo_pool()`
/// expect only validated `tx`s to be passed in.  `ValidatedTx` is a newtype which expresses this
/// precondition to the typesystem.
#[derive(Debug)]
pub struct ValidatedTx<'tx>(&'tx Tx);

impl<'tx> ValidatedTx<'tx> {
    pub const fn new(tx: &'tx Tx) -> Self {
        Self(tx)
    }
}

impl Deref for ValidatedTx<'_> {
    type Target = Tx;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
