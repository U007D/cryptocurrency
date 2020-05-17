mod tx_and_fee;

use crate::{utxo_pool::UtxoPool, OutputTx, Tx, TxHandler};
use rust_decimal::Decimal;
use std::{collections::BinaryHeap, ops::Deref};
use tx_and_fee::TxAndFee;

#[derive(Debug)]
pub struct MaxFeeTxHandler(TxHandler);

impl MaxFeeTxHandler {
    #[must_use]
    pub fn new(utxo_pool: &UtxoPool) -> Self {
        Self(TxHandler::new(utxo_pool))
    }

    // While the provided instructions (below) are difficult to clearly comprehend, for the purposes
    // of this exercise, they are being interpreted to mean "sort the vector or valid transactions
    // in order of greatest to least transaction fees, where a transaction fee is defined by the
    // excess value of the input txs over the output txs in any given `Tx`".
    /// Extra Credit: Create a second file called MaxFeeTxHandler.java whose handleTxs() method
    /// finds a set of transactions with maximum total transaction fees -- i.e. maximize the sum
    /// over all transactions in the set of (sum of input values - sum of output values)).
    pub fn handle_txs(&mut self, possible_txs: &[Tx]) -> Vec<Tx> {
        self.0
            .handle_txs(possible_txs)
            .iter()
            .map(|tx| {
                let fee = self
                    .fee(tx)
                    .unwrap_or_else(|| panic!("Overflow error calculating fee for `Tx`: {:?}", tx));
                TxAndFee(tx, fee)
            })
            .fold(BinaryHeap::new(), |mut acc, el| {
                acc.push(el);
                acc
            })
            .iter()
            .map(|tx_and_fee| tx_and_fee.0)
            .cloned()
            .collect()
    }

    #[must_use]
    pub fn fee(&self, tx: &Tx) -> Option<Decimal> {
        self.sum_of_inputs(tx.input_txs()).and_then(|i_sum| {
            Self::sum_of_outputs(tx.output_txs()).and_then(|o_sum| i_sum.checked_sub(o_sum))
        })
    }

    fn sum_of_outputs<'a, O>(otxs: O) -> Option<Decimal>
    where
        O: Iterator<Item = &'a OutputTx>,
    {
        TxHandler::sum_of_outputs(otxs)
    }
}

impl Deref for MaxFeeTxHandler {
    type Target = TxHandler;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
