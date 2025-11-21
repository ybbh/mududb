use crate::data_type::dat_type::DatType;
use crate::data_type::dt_fn_param::{ErrParam, FnParam};
use crate::data_type::dtp_array::DTPArray;

pub fn fn_array_param_in(params: &str) -> Result<DatType, ErrParam> {
    let param = serde_json::from_str::<DTPArray>(params)
        .map_err(|err| ErrParam::ParamParseError(err.to_string()))?;
    Ok(DatType::from_array(param))
}

pub const FN_ARRAY_PARAM: FnParam = FnParam {
    input: fn_array_param_in,
    default: None,
};
