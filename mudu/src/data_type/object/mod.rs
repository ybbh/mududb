use crate::data_type::dat_type::DatType;
use crate::data_type::dt_impl::dt_create;

pub fn new_object_type(name: String, fields: Vec<(String, DatType)>) -> DatType {
    dt_create::create_object_type(name, fields)
}