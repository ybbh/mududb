use crate::common::result::RS;
use crate::error::ec::EC;
use crate::m_error;
use crate::tuple::datum_desc::DatumDesc;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub type JsonNumber = serde_json::Number;
pub type JsonValue = serde_json::Value;
pub type JsonMap<K, V> = serde_json::Map<K, V>;
pub type JsonArray = Vec<JsonValue>;


#[macro_export]
macro_rules! json_value {
    // Hide distracting implementation details from the generated rustdoc.
    ($($json:tt)+) => {
        serde_json::json!($($json)+)
    };
}

pub fn binary_to_json(binary: &[u8], desc: &DatumDesc) -> RS<JsonValue> {
    let obj = desc.dat_type();
    let param = obj;
    let tp_id = desc.dat_type_id();
    let (dat_internal, _) = tp_id.fn_recv()(binary, param)
        .map_err(|e| m_error!(EC::TypeBaseErr, "error when generating default value", e))?;
    let dat_printable = tp_id.fn_output_json()(&dat_internal, param)
        .map_err(|e| m_error!(EC::TypeBaseErr, "error when converting to printable", e))?;
    let value = dat_printable.into_json_value();
    Ok(value)
}

pub fn to_json_str<S: Serialize>(value: &S) -> RS<String> {
    serde_json::to_string(value)
        .map_err(|e| m_error!(EC::EncodeErr, "error when encoding json", e))
}

pub fn from_json_str<D: DeserializeOwned>(s: &str) -> RS<D> {
    serde_json::from_str(s).map_err(|e| m_error!(EC::DecodeErr, "error when decoding json", e))
}
