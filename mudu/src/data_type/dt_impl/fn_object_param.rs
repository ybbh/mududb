use crate::data_type::dat_type::DatType;
use crate::data_type::dt_fn_param::{ErrParam, FnParam};
use crate::data_type::dtp_object::DTPObject;
use crate::utils;

pub fn fn_object_param_in(s: &str) -> Result<DatType, ErrParam> {
    let param: DTPObject = utils::json::from_json_str(s)
        .map_err(|e| { ErrParam::ParamParseError(format!("parse parameter json error {}", e)) })?;
    Ok(DatType::from_object(param))
}

pub const FN_OBJECT_PARAM: FnParam = FnParam {
    input: fn_object_param_in,
    default: None,
};
