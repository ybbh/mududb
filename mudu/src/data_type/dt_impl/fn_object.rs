use crate::data_type::dat_binary::DatBinary;
use crate::data_type::dat_json::DatJson;
use crate::data_type::dat_textual::DatTextual;
use crate::data_type::dat_type::DatType;
use crate::data_type::dat_type_id::DatTypeID;
use crate::data_type::dat_value::DatValue;
use crate::data_type::dt_fn_convert::{ErrFnBase, FnBase};
use crate::data_type::dtp_object::DTPObject;
use crate::data_type::dvi_object::DVIObject;
use crate::utils::bin_size::BinSize;
use crate::utils::bin_slot::BinSlot;
use crate::utils::json::{from_json_str, JsonMap, JsonValue};

pub fn fn_object_in(s: &str, dat_type: &DatType) -> Result<DatValue, ErrFnBase> {
    let json_value: JsonValue = from_json_str(s).map_err(|e| {
        ErrFnBase::ErrTypeConvert(e.to_string())
    })?;
    let dat = fn_object_in_json(&json_value, dat_type)?;
    Ok(dat)
}

pub fn fn_object_out(v: &DatValue, dat_type: &DatType) -> Result<DatTextual, ErrFnBase> {
    let json = fn_object_out_json(v, dat_type)?;
    Ok(DatTextual::from(json.to_string()))
}


pub fn fn_object_in_json(json: &JsonValue, ty: &DatType) -> Result<DatValue, ErrFnBase> {
    let param = object_param(ty);
    let opt_object = json.as_object();
    let map = match opt_object {
        Some(map) => { map }
        None => { return Err(ErrFnBase::ErrTypeConvert("".to_string())) }
    };
    let field_name_ty = param.fields();

    let mut object_fields = Vec::with_capacity(map.len());
    for (name, ty) in field_name_ty {
        let opt_v = map.get(name);
        let field_json = match opt_v {
            None => {
                return Err(ErrFnBase::ErrTypeConvert("convert error".to_string()))
            }
            Some(v) => { v }
        };
        let id = ty.dat_type_id();
        let dat_val = id.fn_input_json()(field_json, ty)?;
        object_fields.push(dat_val);
    }
    Ok(DatValue::from_object(DVIObject::new(object_fields)))
}


pub fn fn_object_out_json(v: &DatValue, dt: &DatType) -> Result<DatJson, ErrFnBase> {
    let param = object_param(dt);
    let datum_object: &DVIObject = v.expect_object();
    if datum_object.fields().len() != param.fields().len() {
        return Err(ErrFnBase::ErrTypeConvert("convert error".to_string()));
    }
    let mut json_map = JsonMap::with_capacity(datum_object.fields().len());
    for (i, dat_value) in datum_object.fields().iter().enumerate() {
        let (name, ty) = &param.fields()[i];
        let id = ty.dat_type_id();
        let field_json = id.fn_output_json()(dat_value, ty)?;
        json_map.insert(name.clone(), field_json.into_json_value());
    };
    Ok(DatJson::from(JsonValue::Object(json_map)))
}

pub fn fn_object_len(_: &DatType) -> Result<Option<u32>, ErrFnBase> {
    Ok(None)
}


fn header_size(num_field: usize) -> usize {
    BinSize::size_of() + BinSlot::size_of() * num_field
}

pub fn fn_object_dat_output_len(dat_value: &DatValue, dat_type: &DatType) -> Result<u32, ErrFnBase> {
    let param = object_param(dat_type);
    let mut size = header_size(param.fields().len()) as u32;
    let datum_object: &DVIObject = dat_value.expect_object();
    if datum_object.fields().len() != param.fields().len() {
        return Err(ErrFnBase::ErrTypeConvert("convert error".to_string()));
    }
    for (i, (_, ty)) in param.fields().iter().enumerate() {
        let id = ty.dat_type_id();
        let field_dat_value = &datum_object.fields()[i];
        let n = id.fn_send_dat_len()(field_dat_value, ty)?;
        size += n;
    }
    Ok(size)
}

pub fn fn_object_send(value: &DatValue, dat_type: &DatType) -> Result<DatBinary, ErrFnBase> {
    let size = fn_object_dat_output_len(value, dat_type)?;
    let mut vec = Vec::with_capacity(size as usize);
    unsafe { vec.set_len(size as usize) };
    fn_object_send_to(value, dat_type, &mut vec)?;
    Ok(DatBinary::from(vec))
}

pub fn fn_object_send_to(value: &DatValue, dat_type: &DatType, buf: &mut [u8]) -> Result<u32, ErrFnBase> {
    let param = object_param(dat_type);
    let datum_object: &DVIObject = value.expect_object();
    if datum_object.fields().len() != param.fields().len() {
        return Err(ErrFnBase::ErrTypeConvert("convert error".to_string()));
    }
    let hdr_size = header_size(param.fields().len());
    if buf.len() < hdr_size {
        let len = fn_object_dat_output_len(value, dat_type)?;
        return Err(ErrFnBase::ErrLowBufSpace(len));
    }
    let mut offset = hdr_size;
    for (i, field_value) in datum_object.fields().iter().enumerate() {
        let (_, ty) = &param.fields()[i];
        let id = ty.dat_type_id();
        let write_n = id.fn_send_to()(field_value, ty, &mut buf[offset..])?;
        let bin_slot = BinSlot::new(offset as u32, write_n);
        let slot_off = BinSize::size_of() + BinSlot::size_of() * i;
        bin_slot.copy_to_slice(&mut buf[slot_off..]);
        offset += write_n as usize;
    }
    // write the total length of the send binary data
    let bin_size = BinSize::new(offset as u32);
    bin_size.copy_to_slice(&mut buf[..BinSize::size_of()]);
    Ok(offset as u32)
}

pub fn fn_object_recv(binary: &[u8], dat_type: &DatType) -> Result<(DatValue, u32), ErrFnBase> {
    let param = object_param(dat_type);
    let hdr_size = header_size(param.fields().len());
    let size = BinSize::from_slice(binary).size();
    if size as usize > binary.len() || hdr_size > binary.len() {
        return Err(ErrFnBase::ErrLengthError);
    };
    let mut vec_fields = Vec::with_capacity(param.fields().len());
    for (i, (_, ty)) in param.fields().iter().enumerate() {
        let id = ty.dat_type_id();
        let slot_off = BinSize::size_of() + BinSlot::size_of() * i;
        let slot = BinSlot::from_slice(&binary[slot_off..slot_off + BinSlot::size_of()]);
        let (dat_value, _) = id.fn_recv()(&binary[slot.offset() as usize..(slot.offset() + slot.length()) as usize], ty)?;
        vec_fields.push(dat_value);
    }
    Ok((DatValue::from_object(DVIObject::new(vec_fields)), size))
}


pub fn fn_object_default(ty: &DatType) -> Result<DatValue, ErrFnBase> {
    if ty.dat_type_id() != DatTypeID::Object {
        return Err(ErrFnBase::ErrTypeConvert("convert error".to_string()));
    }
    let mut fields = Vec::new();
    let param = object_param(ty);
    for (_field, field_ty) in param.fields() {
        let value = field_ty.dat_type_id().fn_default()(field_ty)?;
        fields.push(value);
    }
    let dvi_obj = DVIObject::new(fields);
    Ok(DatValue::from_object(dvi_obj))
}

fn object_param(dat_type: &DatType) -> &DTPObject {
    dat_type.expect_object_param()
}

pub const FN_OBJECT_CONVERT: FnBase = FnBase {
    input_textual: fn_object_in,
    output_textual: fn_object_out,
    input_json: fn_object_in_json,
    output_json: fn_object_out_json,
    type_len: fn_object_len,
    data_len: fn_object_dat_output_len,
    receive: fn_object_recv,
    send: fn_object_send,
    send_to: fn_object_send_to,
    default: fn_object_default,
};
