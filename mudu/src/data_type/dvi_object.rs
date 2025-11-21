use crate::common::result::RS;
use crate::data_type::dat_binary::DatBinary;
use crate::data_type::dat_textual::DatTextual;
use crate::data_type::dat_type_id::DatTypeID;
use crate::data_type::dat_value::DatValue;
use crate::data_type::datum::DatumDyn;
use crate::data_type::dt_fn_param::DatType;
use crate::error::ec::EC;
use crate::m_error;
use std::sync::Arc;
use crate::data_type::dat_value_inner::DatValueInner;


/// Data Value Inner Object Type
#[derive(Debug, Clone)]
pub struct DVIObject {
    inner: Arc<Vec<DatValue>>,
}

impl DVIObject {
    pub fn new(vec: Vec<DatValue>) -> Self {
        Self { inner: Arc::new(vec) }
    }

    pub fn fields(&self) -> &[DatValue] {
        self.inner.as_slice()
    }
}

impl DatValueInner for DVIObject {

}

impl DatumDyn for DVIObject {
    fn dat_type_id(&self) -> RS<DatTypeID> {
        Ok(DatTypeID::Object)
    }

    fn to_binary(&self, param: &DatType) -> RS<DatBinary> {
        let internal = DatValue::from_object(self.clone());
        DatTypeID::Object.fn_send()(&internal, param)
            .map_err(|e| m_error!(EC::TypeBaseErr, "object type, convert data format error", e))
    }

    fn to_textual(&self, param: &DatType) -> RS<DatTextual> {
        let internal = DatValue::from_object(self.clone());
        DatTypeID::Object.fn_output()(&internal, param)
            .map_err(|e| m_error!(EC::TypeBaseErr, "object type, convert data format error", e))
    }

    fn to_value(&self, _param: &DatType) -> RS<DatValue> {
        let internal = DatValue::from_object(self.clone());
        Ok(internal)
    }

    fn clone_boxed(&self) -> Box<dyn DatumDyn> {
        Box::new(self.clone())
    }
}