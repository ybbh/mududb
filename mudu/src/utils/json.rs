use crate::common::result::RS;
use crate::data_type::dt_impl::dat_type_id::DatTypeID;
use crate::error::ec::EC;
use crate::m_error;
use crate::tuple::datum_desc::DatumDesc;
use serde_json::Value;

pub fn binary_to_json(binary: &[u8], desc: &DatumDesc) -> RS<Value> {
    let param = desc.param_obj();
    let tp_id = desc.dat_type_id();
    let dat_internal = tp_id.fn_recv()(binary, param)
        .map_err(|e| m_error!(EC::TypeBaseErr, "error when generating default value", e))?;
    let dat_printable = tp_id.fn_output()(&dat_internal, param)
        .map_err(|e| m_error!(EC::TypeBaseErr, "error when converting to printable", e))?;
    let s = dat_printable.into();
    let value = if tp_id == DatTypeID::CharFixedLen || tp_id == DatTypeID::CharVarLen {
        Value::String(s)
    } else {
        s.parse()
            .map_err(|e| m_error!(EC::DecodeErr, "error when generating default value", e))?
    };
    Ok(value)
}
