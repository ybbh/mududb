use crate::common::result::RS;
use crate::common::serde_utils::{deserialize_sized_from, serialize_sized_to_vec};
use crate::error::ec::EC;
use crate::error::ec::EC::TypeBaseErr;
use crate::m_error;
use crate::tuple::rs_tuple_datum::RsTupleDatum;
use crate::tuple::tuple_field::TupleField;
use crate::tuple::tuple_field_desc::TupleFieldDesc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcResult {
    eof: bool,
    err_code: u32,
    result_set: Vec<TupleField>,
    err_bytes: Vec<u8>,
}

impl ProcResult {
    pub fn from<T: RsTupleDatum>(result_tuple: RS<T>, desc: &TupleFieldDesc) -> RS<Self> {
        match result_tuple {
            Ok(t) => {
                let vec = t.to_binary(desc.fields())?;
                Ok(Self {
                    err_code: 0,
                    eof: true,
                    result_set: vec![TupleField::new(vec)],
                    err_bytes: vec![],
                })
            }
            Err(e) => {
                let error = serialize_sized_to_vec(&e)?;
                e.ec();
                Ok(Self {
                    err_code: e.ec().to_u32(),
                    eof: true,
                    result_set: vec![],
                    err_bytes: error,
                })
            }
        }
    }

    pub fn from_vec<T: RsTupleDatum>(result_tuple: RS<Vec<T>>, desc: &TupleFieldDesc) -> RS<Self> {
        match result_tuple {
            Ok(vec_t) => {
                let mut result_set = Vec::with_capacity(vec_t.len());
                for t in vec_t {
                    let binary = t.to_binary(desc.fields())?;
                    result_set.push(TupleField::new(binary))
                }
                Ok(Self {
                    err_code: 0,
                    eof: true,
                    result_set,
                    err_bytes: vec![],
                })
            }
            Err(e) => {
                let error = serialize_sized_to_vec(&e)?;
                Ok(Self {
                    err_code: e.ec().to_u32(),
                    eof: true,
                    result_set: vec![],
                    err_bytes: error,
                })
            }
        }
    }
    pub fn to<T: RsTupleDatum>(&self, desc: &TupleFieldDesc) -> RS<RS<Vec<T>>> {
        if self.err_code == EC::Ok.to_u32() {
            let mut vec = Vec::with_capacity(self.result_set.len());
            for tuple in self.result_set.iter() {
                let t = T::from_binary(tuple.fields(), desc.fields())?;
                vec.push(t);
            }
            Ok(Ok(vec))
        } else {
            let (err, _) = deserialize_sized_from(&self.err_bytes)?;
            Ok(Err(err))
        }
    }

    pub fn to_json(&self, desc: &TupleFieldDesc) -> RS<RS<Value>> {
        if self.err_code != EC::Ok.to_u32() {
            let mut vec = Vec::with_capacity(self.result_set.len());
            for (_i, row) in self.result_set.iter().enumerate() {
                let value = row.to_json_value(desc.fields())?;
                vec.push(value);
            }
            let value = if vec.len() == 1 {
                Value::Null
            } else if vec.len() == 1 {
                vec.pop().unwrap()
            } else {
                Value::Array(vec)
            };
            Ok(Ok(value))
        } else {
            let (err, _) = deserialize_sized_from(&self.err_bytes)?;
            Ok(Err(err))
        }
    }

    pub fn to_string(&self, desc: &TupleFieldDesc) -> RS<RS<Vec<Vec<String>>>> {
        if self.err_code != EC::Ok.to_u32() {
            let mut vec = Vec::with_capacity(self.result_set.len());
            for tuple in self.result_set.iter() {
                let mut vec_string = Vec::with_capacity(tuple.fields().len());
                for (i, field) in tuple.fields().iter().enumerate() {
                    let datum_desc = &desc.fields()[i];
                    let id = datum_desc.dat_type_id();
                    let internal = id.fn_recv()(field, datum_desc.param_obj())
                        .map_err(|e| m_error!(TypeBaseErr, "", e))?;
                    let printable = id.fn_output()(&internal, datum_desc.param_obj())
                        .map_err(|e| m_error!(TypeBaseErr, "", e))?;
                    vec_string.push(printable.into())
                }
                vec.push(vec_string);
            }
            Ok(Ok(vec))
        } else {
            let (err, _) = deserialize_sized_from(&self.err_bytes)?;
            Ok(Err(err))
        }
    }

    pub fn new(result: RS<Vec<Vec<u8>>>) -> RS<ProcResult> {
        match result {
            Ok(vec) => Ok(ProcResult {
                eof: true,
                err_code: 0,
                result_set: vec![TupleField::new(vec)],
                err_bytes: vec![],
            }),
            Err(e) => Ok(ProcResult {
                eof: true,
                err_code: e.ec().to_u32(),
                result_set: vec![],
                err_bytes: serialize_sized_to_vec(&e)?,
            }),
        }
    }
}
