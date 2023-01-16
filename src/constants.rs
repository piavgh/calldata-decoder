pub const MASK_4: &str = "FFFFFFFF";

// PUSH20 followed by AND is used to "mask" the 32-byte address into its correct type.
pub const MASK_20: &str = "ffffffffffffffffffffffffffffffffffffffff";

pub const EMPTY_4: &str = "00000000";
pub const EMPTY_32: &str = "0000000000000000000000000000000000000000000000000000000000000000";

//
pub const MAX_U256: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

//
pub const MAX_U128: &str = "00000000000000000000000000000000ffffffffffffffffffffffffffffffff";


#[derive(Debug, Clone)]
pub enum Types {
    AnyZero,
    AnyMax,
    Uint,
    Int,
    Bytes,
    Bool,
    Uint8,
    Bytes1,
    Bytes20,
    Address,
    Selector,
    String,
    Address0,
    ZeroUint,
    MaxUint128,
}
