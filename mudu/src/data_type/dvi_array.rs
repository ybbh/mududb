use crate::common::result::RS;
use crate::data_type::dat_binary::DatBinary;
use crate::data_type::dat_textual::DatTextual;
use crate::data_type::dat_type_id::DatTypeID;
use crate::data_type::dat_value::DatValue;
use crate::data_type::dat_value_inner::DatValueInner;
use crate::data_type::datum::{Datum, DatumDyn};
use crate::data_type::dt_fn_param::DatType;
use crate::error::ec::EC;
use crate::m_error;

/// Data Value Inner Array Type
#[derive(Debug, Clone)]
pub struct DVIArray {
    pub inner: Vec<DatValue>,
}

#[derive(Debug, Clone)]
pub struct DatumArrayT<D: Datum> {
    pub inner: Vec<D>,
}

impl DVIArray {
    pub fn new(vec: Vec<DatValue>) -> DVIArray {
        Self { inner: vec }
    }

    pub fn array(&self) -> &[DatValue] {
        self.inner.as_slice()
    }
}

impl DatValueInner for DVIArray {

}
impl DatumDyn for DVIArray {
    fn dat_type_id(&self) -> RS<DatTypeID> {
        Ok(DatTypeID::Array)
    }


    fn to_binary(&self, param: &DatType) -> RS<DatBinary> {
        let internal = DatValue::from_array(self.clone());
        DatTypeID::Array.fn_send()(&internal, param)
            .map_err(|e| m_error!(EC::TypeBaseErr, "array data format error", e))
    }

    fn to_textual(&self, type_obj: &DatType) -> RS<DatTextual> {
        let internal = DatValue::from_array(self.clone());
        DatTypeID::Array.fn_output()(&internal, type_obj)
            .map_err(|e| m_error!(EC::TypeBaseErr, "array type, convert data format error", e))
    }

    fn to_value(&self, _param: &DatType) -> RS<DatValue> {
        let internal = DatValue::from_array(self.clone());
        Ok(internal)
    }

    fn clone_boxed(&self) -> Box<dyn DatumDyn> {
        Box::new(self.clone())
    }
}


impl DVIArray {
    
}


