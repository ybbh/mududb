use crate::common::result::RS;
use crate::tuple::datum_desc::DatumDesc;
use crate::tuple::tuple_field_desc::TupleFieldDesc;

pub trait EnumerableDatum {
    fn to_binary(&self, datum_desc: &[DatumDesc]) -> RS<Vec<Vec<u8>>>;

    fn tuple_desc(&self, field_name: &[String]) -> RS<TupleFieldDesc>;
}
