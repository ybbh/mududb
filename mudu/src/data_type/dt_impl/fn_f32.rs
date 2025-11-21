use crate::common::endian;
use crate::data_type::dat_binary::DatBinary;
use crate::data_type::dat_json::DatJson;
use crate::data_type::dat_textual::DatTextual;
use crate::data_type::dat_type::DatType;
use crate::data_type::dat_value::DatValue;
use crate::data_type::dt_fn_convert::{ErrFnBase, FnBase};
use crate::json_value;
use crate::utils::json::{from_json_str, JsonValue};

pub fn fn_f32_in_textual(v: &str, _dt: &DatType) -> Result<DatValue, ErrFnBase> {
    let json = from_json_str::<JsonValue>(v).map_err(|e| {
        ErrFnBase::ErrTypeConvert(e.to_string())
    })?;
    fn_f32_in_json(&DatJson::from(json), _dt)
}

pub fn fn_f32_out_textual(v: &DatValue, _dt: &DatType) -> Result<DatTextual, ErrFnBase> {
    let json = fn_f32_out_json(v, _dt)?;
    Ok(DatTextual::from(json.to_string()))
}

pub fn fn_f32_in_json(v: &JsonValue, _: &DatType) -> Result<DatValue, ErrFnBase> {
    let opt_num = v.as_number();
    let opt_f64 = match opt_num {
        Some(num) => num.as_f64(),
        None => { return Err(ErrFnBase::ErrTypeConvert(format!("cannot convert json {} to f32", v.to_string()))) }
    };
    match opt_f64 {
        Some(num) => Ok(DatValue::from_f64(num)),
        None => { Err(ErrFnBase::ErrTypeConvert(format!("cannot convert json {} to f32", v.to_string()))) }
    }
}

pub fn fn_f32_out_json(v: &DatValue, _: &DatType) -> Result<DatJson, ErrFnBase> {
    let i = v.to_f32();
    let json = json_value!(i);
    Ok(DatJson::from(json))
}

pub fn fn_f32_type_output_len(_: &DatType) -> Result<Option<u32>, ErrFnBase> {
    Ok(Some(size_of::<f32>() as u32))
}

pub fn fn_f32_dat_output_len(_: &DatValue, _ty: &DatType) -> Result<u32, ErrFnBase> {
    Ok(fn_f32_type_output_len(_ty)?.unwrap())
}

pub fn fn_f32_send(v: &DatValue, _: &DatType) -> Result<DatBinary, ErrFnBase> {
    let i = v.to_f32();
    let mut buf = vec![0; size_of_val(&i)];
    endian::write_f32(&mut buf, i);
    Ok(DatBinary::from(buf))
}

pub fn fn_f32_send_to(v: &DatValue, _: &DatType, buf: &mut [u8]) -> Result<u32, ErrFnBase> {
    let i = v.to_f32();
    let len = size_of_val(&i) as u32;
    if size_of_val(&i) < buf.len() {
        return Err(ErrFnBase::ErrLowBufSpace(len));
    }
    endian::write_f32(buf, i);
    Ok(len)
}

pub fn fn_f32_recv(buf: &[u8], _: &DatType) -> Result<(DatValue, u32), ErrFnBase> {
    if size_of::<f32>() < buf.len() {
        return Err(ErrFnBase::ErrLowBufSpace(size_of::<f32>() as u32));
    };
    let i = endian::read_f32(buf);
    Ok((DatValue::from_f32(i), size_of::<f32>() as u32))
}

pub fn fn_f32_default(_: &DatType) -> Result<DatValue, ErrFnBase> {
    Ok(DatValue::from_f32(f32::default()))
}

pub const FN_F32_CONVERT: FnBase = FnBase {
    input_textual: fn_f32_in_textual,
    output_textual: fn_f32_out_textual,
    input_json: fn_f32_in_json,
    output_json: fn_f32_out_json,
    type_len: fn_f32_type_output_len,
    data_len: fn_f32_dat_output_len,
    receive: fn_f32_recv,
    send: fn_f32_send,
    send_to: fn_f32_send_to,
    default: fn_f32_default,
};
