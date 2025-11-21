use crate::data_type::dat_type_id::DatTypeID;
use crate::data_type::dt_impl;

pub fn rust_type_to_dt_id(name: &str) -> Option<(DatTypeID, Vec<DatTypeID>)> {
    dt_impl::lang::rust::dt_lang_name_to_id(name)
}

pub fn dt_id_to_rust_name(id: DatTypeID) -> Option<String> {
    dt_impl::lang::rust::dt_id_to_lang_name(id)
}
