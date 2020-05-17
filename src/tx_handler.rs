mod max_fee_tx_handler;
mod validated_tx;

use crate::{utxo_pool::UtxoPool, InputTx, OutputTx, Tx};
pub use max_fee_tx_handler::MaxFeeTxHandler;
use rust_decimal::Decimal;
use std::collections::HashSet;
pub use validated_tx::ValidatedTx;

#[derive(Debug)]
pub struct TxHandler {
    utxo_pool: UtxoPool,
}

impl TxHandler {
    /// Creates a public ledger whose current `UtxoPool` (collection of unspent transaction
    /// outputs) is `utxo_pool`. This should make a copy of `utxo_pool` by using the
    /// `UTXOPool::clone()` method.
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new(utxo_pool: &UtxoPool) -> Self {
        Self {
            utxo_pool: utxo_pool.clone(),
        }
    }

    fn all_claimed_outputs_exist(&self, tx: &Tx) -> bool {
        tx.input_txs().all(|itx| match itx {
            InputTx::Unsigned { output_utxo: utxo }
            | InputTx::Signed {
                output_utxo: utxo,
                signature: _,
            } => self.utxo_pool.contains(utxo),
            _ => false,
        })
    }

    fn all_input_signatures_valid(&self, tx: &Tx) -> bool {
        tx.input_txs().all(|itx| match itx {
            InputTx::Signed {
                output_utxo: utxo,
                signature,
            } => self
                .utxo_pool
                .tx_output(utxo)
                .and_then(|otx| otx.address().verify(&utxo.as_vec(), signature).ok())
                .map_or_else(|| false, |_| true),
            _ => false,
        })
    }

    fn all_output_values_are_non_negative(tx: &Tx) -> bool {
        tx.output_txs().all(|otx| otx.value() >= Decimal::from(0))
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
        self.all_claimed_outputs_exist(tx)
            && self.all_input_signatures_valid(tx)
            && Self::no_utxo_is_multiply_claimed(tx)
            && Self::all_output_values_are_non_negative(tx)
            && self.sum_of_inputs_ge_sum_of_outputs(tx)
    }

    /// Handles each epoch by receiving an unordered array of proposed transactions, checking each
    /// transaction for correctness, returning a mutually valid array of accepted transactions, and
    /// updating the current UTXO pool as appropriate.
    #[must_use]
    pub fn handle_txs(&mut self, possible_txs: &[Tx]) -> Vec<Tx> {
        let valid_txs = possible_txs
            .iter()
            .filter(|tx| self.is_valid_tx(tx))
            .cloned()
            .collect::<Vec<_>>();
        valid_txs
            .iter()
            .map(|tx| ValidatedTx::new(tx))
            .for_each(|vtx| {
                self.utxo_pool
                    .remove_validated_input_txs(&vtx)
                    .unwrap_or_else(|| {
                        unreachable!(
                            "Valid input tx not found in `utxo_pool` (single-threaded context)."
                        )
                    })
                    .add_validated_output_txs(&vtx)
                    .unwrap_or_else(|err| unreachable!(err));
            });
        valid_txs
    }

    fn no_utxo_is_multiply_claimed(tx: &Tx) -> bool {
        tx.input_txs()
            .try_fold(HashSet::new(), |mut set, itx| match itx {
                InputTx::Signed {
                    output_utxo: utxo,
                    signature: _,
                } => match set.insert(utxo) {
                    true => Some(set),
                    false => None,
                },
                _ => None,
            })
            .map_or_else(|| false, |_| true)
    }

    fn sum_of_inputs<'a, I>(&self, mut itxs: I) -> Option<Decimal>
    where
        I: Iterator<Item = &'a InputTx>,
    {
        itxs.try_fold(Decimal::from(0), |acc, itx| match itx {
            InputTx::Signed {
                output_utxo: utxo,
                signature: _,
            } => self
                .utxo_pool
                .tx_output(utxo)
                .and_then(|otx| acc.checked_add(otx.value())),
            _ => None,
        })
    }

    fn sum_of_inputs_ge_sum_of_outputs(&self, tx: &Tx) -> bool {
        self.sum_of_inputs(tx.input_txs())
            .and_then(|i_sum| Self::sum_of_outputs(tx.output_txs()).map(|o_sum| i_sum >= o_sum))
            .map_or_else(|| false, |ge| ge)
    }

    fn sum_of_outputs<'a, O>(mut otxs: O) -> Option<Decimal>
    where
        O: Iterator<Item = &'a OutputTx>,
    {
        otxs.try_fold(Decimal::from(0), |acc, otx| acc.checked_add(otx.value()))
    }
}
