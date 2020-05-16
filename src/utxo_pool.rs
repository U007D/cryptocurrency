use crate::{tx::OutputTx, Error, Result, Utxo};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
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
    pub fn remove_utxo(&mut self, utxo: &Utxo) -> Option<OutputTx> {
        self.hash_map.remove(utxo)
    }

    /// Return the tx output corresponding to `Utxo` or `None` if `Utxo` is not in the pool
    pub fn tx_output(&self, utxo: &Utxo) -> Option<&OutputTx> {
        self.hash_map.get(utxo)
    }

    /// Return true if `Utxo` is in the pool and false otherwise
    pub fn contains(&self, utxo: &Utxo) -> bool {
        self.hash_map.contains_key(utxo)
    }

    /// Returns an iterator of all UTXO's in the pool
    pub fn all_utxos(&self) -> impl Iterator<Item = &Utxo> {
        self.hash_map.iter().map(|(k, _)| k)
    }
}
