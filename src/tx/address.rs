use serde::Serialize;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Address(Vec<u8>);
