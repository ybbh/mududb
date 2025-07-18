use crate::common::result::RS;
use crate::database::attr_datum::AttrDatum;
use crate::tuple::datum::Datum;

pub trait Attribute<T> : AttrDatum + Sized {
    fn from_datum(datum:&Datum) -> RS<Self>;
    
    fn table_name() -> &'static str;

    fn column_name() -> &'static str;
    
    fn is_null(&self) -> bool;

    fn get_opt_value(&self) -> Option<T>;
    
    fn set_opt_value(&mut self, value:Option<T>);

    fn get_value(&self) -> T;
    
    fn set_value(&mut self, value:T);
}