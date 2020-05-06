#![allow(dead_code)]
pub const ERR_ADD_DUPLICATE_UTXO_ATTEMPTED: &str =
    "Error: Attempted to add duplicate `Utxo` key to the `UtxoPool`";
pub const ERR_CANNOT_SIGN_GENESIS: &str = "Error: It is not possible to sign the Genesis block";
pub const ERR_CANNOT_UNSIGN_GENESIS: &str = "Error: It is not possible to unsign the Genesis block";
pub const ERR_CANNOT_BUILD_TX_WO_INPUT_TXS: &str = "Error: There must be at least one Input \
Transaction to build a Transaction";
pub const ERR_CANNOT_BUILD_TX_WO_OUTPUT_TXS: &str = "Error: There must be at least one Output \
Transaction to build a Transaction";
pub const ERR_INTERNAL_SERIALIZATION: &str = "Internal error: Conversion of in-memory data \
structure to raw bytes failed";
pub const ERR_TX_IDX_OUT_OF_BOUNDS: &str = "Error: Supplied Transaction Index is out of bounds";
pub const ERR_INTERNAL: &str = "Internal error (bug)";
pub const ERR_KEY_CONSTRUCTION: &str = "Error constructing key from byte slice";
pub const ERR_SIGNATURE: &str = "Error constructing signature";
