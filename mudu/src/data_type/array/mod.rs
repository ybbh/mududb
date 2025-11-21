use crate::data_type::dat_type::DatType;
use crate::data_type::dt_impl::dt_create;

pub fn new_array_type(inner_type: DatType) -> DatType {
    dt_create::create_array_type(inner_type)
}