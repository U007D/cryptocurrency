use serde::Serialize;
use std::mem::size_of;

pub type TxIdxTyp = u64;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TxIdx(TxIdxTyp);

impl TxIdx {
    // `.to_ne_bytes()` not stable as `const fn`
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn as_arr(self) -> [u8; size_of::<TxIdxTyp>()] {
        self.0.to_ne_bytes()
    }
}

impl From<TxIdx> for usize {
    #[allow(clippy::cast_possible_truncation)]
    fn from(idx: TxIdx) -> Self {
        // TODO: Temporary cast until backing lists are refactored to `HashMaps`
        idx.0 as Self
    }
}
