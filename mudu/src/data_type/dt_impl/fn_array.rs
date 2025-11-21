use crate::data_type::dat_binary::DatBinary;
use crate::data_type::dat_json::DatJson;
use crate::data_type::dat_textual::DatTextual;
use crate::data_type::dat_type::DatType;
use crate::data_type::dat_value::DatValue;
use crate::data_type::dvi_array::DVIArray;
use crate::data_type::dt_fn_convert::{ErrFnBase, FnBase};
use crate::data_type::dtp_array::DTPArray;
use crate::utils::bin_size::BinSize;
use crate::utils::json::{from_json_str, JsonValue};

pub fn fn_array_in(s: &str, dat_type: &DatType) -> Result<DatValue, ErrFnBase> {
    let json_value: JsonValue = from_json_str(s).map_err(|e| {
        ErrFnBase::ErrTypeConvert(e.to_string())
    })?;
    let dat = fn_array_in_json(&json_value, dat_type)?;
    Ok(dat)
}

pub fn fn_array_out(v: &DatValue, dat_type: &DatType) -> Result<DatTextual, ErrFnBase> {
    let json = fn_array_out_json(v, dat_type)?;
    Ok(DatTextual::from(json.to_string()))
}


pub fn fn_array_in_json(json: &JsonValue, ty: &DatType) -> Result<DatValue, ErrFnBase> {
    let param = array_param(ty);
    let opt_array = json.as_array();
    let array = match opt_array {
        Some(array) => { array }
        None => { return Err(ErrFnBase::ErrTypeConvert("".to_string())) }
    };
    let arr_elem_ty = param.dat_type();
    let arr_elem_ty_id = arr_elem_ty.dat_type_id();
    let mut value_array = Vec::with_capacity(array.len());
    for v in array.iter() {
        let dat_value = arr_elem_ty_id.fn_input_json()(v, &arr_elem_ty)?;
        value_array.push(dat_value);
    };
    Ok(DatValue::from_array(DVIArray::new(value_array)))
}


pub fn fn_array_out_json(v: &DatValue, dt: &DatType) -> Result<DatJson, ErrFnBase> {
    let param = array_param(dt);
    let datum_array: &DVIArray = v.expect_array();
    let arr_elem_ty = param.dat_type();
    let arr_elem_ty_id = arr_elem_ty.dat_type_id();
    let mut vec_json_value = Vec::with_capacity(datum_array.array().len());
    for v in datum_array.array().iter() {
        let dat_value = arr_elem_ty_id.fn_output_json()(v, &arr_elem_ty)?;
        vec_json_value.push(dat_value.into_json_value());
    };
    Ok(DatJson::from(JsonValue::Array(vec_json_value)))
}

pub fn fn_type_output_len(_: &DatType) -> Result<Option<u32>, ErrFnBase> {
    Ok(None)
}

fn header_size() -> usize {
    BinSize::size_of() * 2
}

pub fn fn_dat_output_len(dat_value: &DatValue, dat_type: &DatType) -> Result<u32, ErrFnBase> {
    let param = array_param(dat_type);
    let datum_array = dat_value.expect_array();
    let mut size = header_size() as u32;
    let ty = param.dat_type();
    let id = ty.dat_type_id();
    for item_dat_value in datum_array.array() {
        let n = id.fn_send_dat_len()(item_dat_value, ty)?;
        size += n;
    }
    Ok(size)
}

fn handle_datum_array_recv(
    n: u32,
    binary: &[u8],
    dat_param: &DatType,
    vec: &mut Vec<DatValue>,
) -> Result<(), ErrFnBase> {
    let mut offset = 0u32;
    for _i in 0..n {
        let (s, bytes) = dat_param.dat_type_id().fn_recv()(&binary[offset as usize..], dat_param)?;
        vec.push(s.into());
        offset += bytes;
    }
    Ok(())
}

pub fn fn_array_send(dat_value: &DatValue, dat_type: &DatType) -> Result<DatBinary, ErrFnBase> {
    let len = fn_dat_output_len(dat_value, dat_type)?;
    let mut vec = Vec::with_capacity(len as usize);
    unsafe { vec.set_len(len as usize); }
    let _ = fn_array_send_to(dat_value, dat_type, &mut vec)?;
    Ok(DatBinary::from(vec))
}

pub fn fn_array_send_to(object: &DatValue, dat_param: &DatType, buf: &mut [u8]) -> Result<u32, ErrFnBase> {
    let param = array_param(dat_param);
    let datum_array: &DVIArray = object.expect_array();
    let hdr_size = header_size();
    let mut offset = hdr_size as u32;
    let ty = param.dat_type();
    let id = ty.dat_type_id();
    for item_dat_value in datum_array.array().iter() {
        let s = id.fn_send_to()(item_dat_value, ty, &mut buf[offset as usize..])?;
        offset += s;
    }
    let binary_bytes = BinSize::new(offset);
    binary_bytes.copy_to_slice(&mut buf[0..BinSize::size_of()]);
    let array_length = BinSize::new(datum_array.array().len() as u32);
    array_length.copy_to_slice(&mut buf[BinSize::size_of()..BinSize::size_of() * 2]);
    Ok(offset)
}

pub fn fn_array_recv(buf: &[u8], dat_param: &DatType) -> Result<(DatValue, u32), ErrFnBase> {
    if buf.len() < header_size() {
        return Err(ErrFnBase::ErrLowBufSpace(0));
    }

    let binary_bytes = BinSize::from_slice(&buf[0..BinSize::size_of()]).size();
    let array_length = BinSize::from_slice(&buf[BinSize::size_of()..BinSize::size_of() * 2]).size();
    if buf.len() < binary_bytes as usize {
        return Err(ErrFnBase::ErrLowBufSpace(binary_bytes));
    }
    let array_param = array_param(dat_param);
    let mut vec_object = Vec::with_capacity(array_length as usize);

    handle_datum_array_recv(
        array_length,
        &buf[header_size()..],
        array_param.dat_type(),
        &mut vec_object,
    )?;
    Ok((DatValue::from_array(DVIArray::new(vec_object)), binary_bytes))
}


pub fn fn_array_default(_: &DatType) -> Result<DatValue, ErrFnBase> {
    let array = DVIArray::new(vec![]);
    Ok(DatValue::from_array(array))
}

fn array_param(dat_type: &DatType) -> &DTPArray {
    dat_type.expect_array_param()
}


pub const FN_ARRAY_CONVERT: FnBase = FnBase {
    input_textual: fn_array_in,
    output_textual: fn_array_out,
    input_json: fn_array_in_json,
    output_json: fn_array_out_json,
    type_len: fn_type_output_len,
    data_len: fn_dat_output_len,
    receive: fn_array_recv,
    send: fn_array_send,
    send_to: fn_array_send_to,
    default: fn_array_default,
};
