use crate::{tx::OutputTx, tx_handler::ValidatedTx, Error, InputTx, Result, TxIdx, Utxo};
use std::{collections::HashMap, convert::TryFrom};

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

    pub fn add_validated_output_txs(&mut self, vtx: &ValidatedTx<'_>) -> Result<&mut Self> {
        vtx.output_txs()
            .enumerate()
            // using `try_fold` instead of `try_for_each()` because the latter returns `impl Try`
            // which is not stable at the time of this writing (1.43.1).
            .try_fold((), |_, (idx, votx)| {
                let tx_hash = vtx.hash().clone();
                let tx_idx = TxIdx::try_from(idx)?;
                self
                    // This panics instead of returning an error, because returning would leave
                    // the `UtxoPool` in an indeterminate state, with some of this `tx`s `otx`s 
                    // added and others not.  Note that `ValidatedTx` exists to prevent this from
                    // ever occurring (in a synchronous context), hence the `unreachable!()`.
                    .add_utxo(Utxo::new(tx_hash, tx_idx), votx.clone()).unwrap_or_else(|err|
                    unreachable!(err));
                Ok(())
            })
            .map(|_| self)
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

    pub fn remove_validated_input_txs(&mut self, vtx: &ValidatedTx<'_>) -> Option<&mut Self> {
        vtx.input_txs()
            // using `try_fold` instead of `try_for_each()` because the latter returns `impl Try`
            // which is not stable at the time of this writing (1.43.1).
            .try_fold((), |_, vitx| match vitx {
                InputTx::Signed {
                    output_utxo: utxo,
                    signature: _,
                } => {
                    self.remove_utxo(utxo);
                    Some(())
                }
                _ => unreachable!("Unexpected non-Signed input tx found."),
            })
            .map(|_| self)
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
