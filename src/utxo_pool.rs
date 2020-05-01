use crate::{tx::OutputTx, Error, Result, Utxo};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UtxoPool {
    /// The current collection of UTXOs, with each one mapped to its corresponding tx
    /// output
    hash_map: HashMap<Utxo, OutputTx>,
}

impl UtxoPool {
    /// Creates a new empty `UtxoPool`
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new(),
        }
    }

    /// Adds a mapping from `Utxo` to a `Transaction::Output` to the pool
    pub fn add_utxo(&mut self, utxo: Utxo, tx_out: OutputTx) -> Result<&mut Self> {
        self.hash_map
            .insert(utxo.clone(), tx_out)
            .ok_or_else(|| Error::AddDuplicateUtxoAttempted(utxo))?;
        Ok(self)
    }

    /// Removes the `Utxo` from the pool
    pub fn remove_utxo(&mut self, utxo: &Utxo) -> Result<OutputTx> {
        self.hash_map
            .remove(utxo)
            .ok_or_else(|| Error::UtxoNotFound(utxo.clone()))
    }

    /// Return the tx output corresponding to `Utxo` or `None` if `Utxo` is not in the pool
    pub fn tx_output(&self, utxo: &Utxo) -> Result<&OutputTx> {
        self.hash_map
            .get(utxo)
            .ok_or_else(|| Error::UtxoNotFound(utxo.clone()))
    }
}
