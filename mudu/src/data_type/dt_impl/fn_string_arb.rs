use crate::data_type::dat_type::DatType;
use crate::data_type::dat_value::DatValue;
use crate::data_type::dt_fn_arbitrary::FnArbitrary;
use crate::data_type::dt_fn_convert::ErrFnBase;
use crate::data_type::dt_impl::dt_create::create_string_type;
use crate::data_type::dtp_string::DTPString;
use arbitrary::{Arbitrary, Unstructured};
use std::any::Any;
use test_utils::_arb_limit::_ARB_MAX_STRING_LEN;
use test_utils::_arb_string::_arbitrary_string;

pub fn param_len(param: &DatType) -> Result<u32, ErrFnBase> {
    if let Some(param) = (param as &dyn Any).downcast_ref::<DTPString>() {
        Ok(param.length())
    } else {
        Err(ErrFnBase::ErrLengthError)
    }
}

pub fn fn_char_arb_val(u: &mut Unstructured, param: &DatType) -> arbitrary::Result<DatValue> {
    let length = param_len(param).unwrap();
    let s = _arbitrary_string(u, length as usize)?;
    DatValue::from_datum(s, param)
        .map_err(|_| arbitrary::Error::IncorrectFormat)
}

pub fn fn_char_arb_printable(u: &mut Unstructured, param: &DatType) -> arbitrary::Result<String> {
    let length = param_len(param).unwrap();
    let s = _arbitrary_string(u, length as usize)?;
    Ok(format!("\"{}\"", s))
}

pub fn fn_char_arb_dt_param(u: &mut Unstructured) -> arbitrary::Result<DatType> {
    let length = u32::arbitrary(u)?;
    let length = length % _ARB_MAX_STRING_LEN as u32;
    Ok(create_string_type(Some(length)))
}

pub const FN_CHAR_FIXED_ARBITRARY: FnArbitrary = FnArbitrary {
    param: fn_char_arb_dt_param,
    value_object: fn_char_arb_val,
    value_print: fn_char_arb_printable,
};
