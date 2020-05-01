use serde::Serialize;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Signature(pub(crate) Vec<u8>);
