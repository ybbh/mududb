use crate::data_type::dt_fn_param::DatType;
use crate::data_type::dtp_array::DTPArray;
use crate::data_type::dtp_object::DTPObject;
use crate::data_type::dtp_string::DTPString;

pub fn create_string_type(opt_length: Option<u32>) -> DatType {
    DatType::from_string(DTPString::new(opt_length.unwrap_or(0)))
}
pub fn create_array_type(inner_type: DatType) -> DatType {
    DatType::from_array(DTPArray::new(inner_type))
}

pub fn create_object_type(name: String, fields: Vec<(String, DatType)>) -> DatType {
    DatType::from_object(DTPObject::new(name, fields))
}