use crate::common::default_value;
use crate::data_type::dat_type::DatType;
use crate::data_type::dt_fn_param::{ErrParam, FnParam};
use crate::data_type::dtp_string::DTPString;

pub fn fn_char_dt_param_in(params: &str) -> Result<DatType, ErrParam> {
    let param: DTPString = serde_json::from_str(params)
        .map_err(|e| {
            ErrParam::ParamParseError(format!("parse parameter error {}", e))
        })?;
    Ok(DatType::from_string(param))
}

pub fn fn_char_dt_param_default() -> DatType {
    let param = DTPString::new(default_value::DT_CHAR_FIXED_LEN_DEFAULT as u32);
    DatType::from_string(param)
}

pub const FN_CHAR_FIXED_PARAM: FnParam = FnParam {
    input: fn_char_dt_param_in,
    default: Some(fn_char_dt_param_default),
};
