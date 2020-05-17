use crate::Tx;
use rust_decimal::Decimal;
use std::cmp::Ordering;

#[derive(Debug)]
pub(super) struct TxAndFee<'tx>(pub(super) &'tx Tx, pub(super) Decimal);

impl Eq for TxAndFee<'_> {}

// Allows for (fee-)ordered insert of (transaction fee, transaction) into `BinaryHeap`
impl Ord for TxAndFee<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialEq for TxAndFee<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for TxAndFee<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
