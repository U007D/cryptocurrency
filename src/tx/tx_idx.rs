use serde::Serialize;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TxIdx(u64);

impl From<TxIdx> for usize {
    #[allow(clippy::cast_possible_truncation)]
    fn from(idx: TxIdx) -> Self {
        // TODO: Temporary cast until backing lists are refactored to `HashMaps`
        idx.0 as Self
    }
}
