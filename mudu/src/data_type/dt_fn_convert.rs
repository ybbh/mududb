use crate::data_type::dat_json::DatJson;
use crate::data_type::{
    dat_binary::DatBinary,
    dat_textual::DatTextual,
    dat_type::DatType,
    dat_value::DatValue,
};
use crate::utils::json::JsonValue;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error types for base function operations
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub enum ErrFnBase {
    ErrLengthError,
    /// Failed to convert between types
    ErrTypeConvert(String),
    /// Insufficient buffer space for operation
    ErrLowBufSpace(u32),
}

impl Display for ErrFnBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ErrFnBase {}

// =============================================================================
// Function Type Definitions
// =============================================================================

/// Converts external textual representation to internal representation
pub type FnInputTextual = fn(&str, &DatType) -> Result<DatValue, ErrFnBase>;

/// Converts internal representation to external textual representation
pub type FnOutputTextual = fn(&DatValue, &DatType) -> Result<DatTextual, ErrFnBase>;

/// Converts external textual representation to internal representation
pub type FnInputJson = fn(&JsonValue, &DatType) -> Result<DatValue, ErrFnBase>;

/// Converts internal representation to external textual representation
pub type FnOutputJson = fn(&DatValue, &DatType) -> Result<DatJson, ErrFnBase>;

/// Returns fixed byte length for fixed-length data types
pub type FnTypeLen = fn(&DatType) -> Result<Option<u32>, ErrFnBase>;

/// Returns byte length for variable-length data types
pub type FnDataLen = fn(&DatValue, &DatType) -> Result<u32, ErrFnBase>;

/// Converts internal representation to external binary representation
pub type FnSend = fn(&DatValue, &DatType) -> Result<DatBinary, ErrFnBase>;

/// Converts internal representation to external binary representation into provided buffer
pub type FnSendTo = fn(&DatValue, &DatType, &mut [u8]) -> Result<u32, ErrFnBase>;

/// Converts external binary representation to internal representation
pub type FnReceive = fn(&[u8], &DatType) -> Result<(DatValue, u32), ErrFnBase>;

/// Provides default value for data type
pub type FnDefault = fn(&DatType) -> Result<DatValue, ErrFnBase>;

// =============================================================================
// Core Function Structure
// =============================================================================

/// Collection of base functions that define data type operations
pub struct FnBase {
    /// Converts text input to internal representation
    pub input_textual: FnInputTextual,
    /// Converts internal representation to text output
    pub output_textual: FnOutputTextual,
    /// Converts JSON input to internal representation
    pub input_json: FnInputJson,
    /// Converts internal representation to JSON output
    pub output_json: FnOutputJson,
    /// Returns fixed length for data type
    pub type_len: FnTypeLen,
    /// Returns byte length for variable-length data type
    pub data_len: FnDataLen,
    /// Receives binary data and converts to internal representation
    pub receive: FnReceive,
    /// Sends internal representation as binary data
    pub send: FnSend,
    /// Sends internal representation to provided buffer
    pub send_to: FnSendTo,
    /// Provides default value for data type
    pub default: FnDefault,
}