use crate::data_type::dat_type::DatType;
use crate::data_type::dt_impl::dt_create;

pub fn new_array_type(opt_length: Option<u32>) -> DatType {
    dt_create::create_string_type(opt_length)
}