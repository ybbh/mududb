use crate::common::cmp_order::Order;
use crate::common::result::RS;
use crate::data_type::dat_type::DatType;
use crate::data_type::dt_param::{DTPDyn, DTPStatic};
use crate::utils;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DTPObject {
    name: String,
    // field name and its data kind
    field: Vec<(String, DatType)>,
}

impl DTPDyn for DTPObject {
    fn clone_boxed(&self) -> Box<dyn DTPDyn> {
        Box::new(self.clone())
    }

    fn de_from_json(&mut self, json: &str) -> RS<()> {
        let s: Self = utils::json::from_json_str::<Self>(json)?;
        self.name = s.name;
        self.field = s.field;
        Ok(())
    }

    fn se_to_json(&self) -> RS<String> {
        utils::json::to_json_str(&self)
    }
}

impl DTPObject {
    pub fn new(name: String, field: Vec<(String, DatType)>) -> DTPObject {
        let mut map = HashMap::new();
        for (name, ty) in &field {
            map.insert(name.clone(), ty.clone());
        }
        Self { name, field }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn fields(&self) -> &Vec<(String, DatType)> {
        &self.field
    }

    fn compare(&self, other: &Self) -> Ordering {
        if self.name.eq(&other.name) {
            Ordering::Equal
        } else {
            self.field.len().cmp(&other.field.len())
        }
    }
}

impl Order for DTPObject {
    fn cmp_ord(&self, other: &Self) -> RS<Ordering> {
        Ok(self.compare(other))
    }
}

impl DTPStatic for DTPObject {}