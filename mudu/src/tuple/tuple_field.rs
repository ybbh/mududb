use crate::common::result::RS;
use crate::error::ec::EC;
use crate::m_error;
use crate::tuple::datum_desc::DatumDesc;
use crate::utils::json::binary_to_json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::map::Map;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TupleField {
    fields: Vec<Vec<u8>>,
}

impl TupleField {
    pub fn new(fields: Vec<Vec<u8>>) -> TupleField {
        Self { fields }
    }

    pub fn fields(&self) -> &Vec<Vec<u8>> {
        &self.fields
    }

    pub fn mut_fields(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.fields
    }

    pub fn get(&self, n: usize) -> Option<Vec<u8>> {
        self.fields.get(n).cloned()
    }

    pub fn to_json_value(&self, desc: &[DatumDesc]) -> RS<Value> {
        if self.fields().len() != desc.len() {
            return Err(m_error!(
                EC::DBInternalError,
                format!(
                    "expected {} fields but got {}",
                    desc.len(),
                    self.fields().len()
                )
            ));
        }
        let mut map = Map::with_capacity(self.fields().len());
        for (i, field) in self.fields().iter().enumerate() {
            let d = &desc[i];
            let json_value = binary_to_json(field, d)?;
            map.insert(d.name().to_owned(), json_value);
        }
        Ok(Value::Object(map))
    }
    pub fn to_printable(&self, desc: &[DatumDesc]) -> RS<Vec<String>> {
        if self.fields().len() != desc.len() {
            return Err(m_error!(
                EC::DBInternalError,
                format!(
                    "expected {} fields but got {}",
                    desc.len(),
                    self.fields().len()
                )
            ));
        }
        let mut vec_string = Vec::with_capacity(self.fields().len());
        for (i, field) in self.fields().iter().enumerate() {
            let datum_desc = &desc[i];
            let id = datum_desc.dat_type_id();
            let internal = id.fn_recv()(field, datum_desc.param_obj())
                .map_err(|e| m_error!(EC::TypeBaseErr, "convert binary to internal error", e))?;
            let printable = id.fn_output()(&internal, datum_desc.param_obj())
                .map_err(|e| m_error!(EC::TypeBaseErr, "convert internal to binary error", e))?;
            vec_string.push(printable.into())
        }
        Ok(vec_string)
    }
}

impl AsRef<TupleField> for TupleField {
    fn as_ref(&self) -> &Self {
        self
    }
}
