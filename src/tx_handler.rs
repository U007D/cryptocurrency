use crate::Tx;

#[derive(Debug)]
pub struct TxHandler {}

impl TxHandler {
    /// Creates a public ledger whose current `UtxoPool` (collection of unspent transaction
    /// outputs) is `utxo_pool`. This should make a copy of `utxo_pool` by using the
    /// `UTXOPool::clone()` method.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        unimplemented!()
    }

    /// return `true` if:
    /// (1) all outputs claimed by `tx` are in the current UTXO pool,
    /// (2) the signatures on each input of `tx` are valid,
    /// (3) no UTXO is claimed multiple times by `tx`,
    /// (4) all of `tx`s output values are non-negative, and
    /// (5) the sum of `tx`s input values is greater than or equal to the sum of its output
    ///     values; and `false` otherwise.
    #[must_use]
    pub fn is_valid_tx(&self, tx: &Tx) -> bool {
        unimplemented!()
    }

    /// Handles each epoch by receiving an unordered array of proposed transactions, checking each
    /// transaction for correctness, returning a mutually valid array of accepted transactions, and
    /// updating the current UTXO pool as appropriate.
    #[must_use]
    pub fn handle_txs(&self, possible_txs: &[Tx]) -> Vec<Tx> {
        unimplemented!()
    }
}
