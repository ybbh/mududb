use crate::data_type::dat_type_id::DatTypeID;
use crate::data_type::dt_fn_param::ErrParam;
use crate::data_type::dt_impl::dat_table::get_fn_param;

use crate::common::cmp_order::Order;
use crate::common::result::RS;
use crate::data_type::dt_info::DTInfo;
use crate::data_type::dtp_array::DTPArray;
use crate::data_type::dtp_kind::DTPKind;
use crate::data_type::dtp_object::DTPObject;
use crate::data_type::dtp_string::DTPString;
use crate::error::ec::EC;
use crate::m_error;
use paste::paste;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;

/// Data type param object
#[derive(Debug)]
pub struct DatType {
    id: DatTypeID,
    param: Option<DTPKind>,
}

impl Clone for DatType {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            param: self.param.clone(),
        }
    }
}

unsafe impl Send for DatType {}

unsafe impl Sync for DatType {}

impl DatType {
    pub fn default_for(id: DatTypeID) -> DatType {
        if !id.is_primitive_type() {
            panic!("DatType::default_for({:?})", id);
        }
        let opt = id.opt_fn_param();
        let param_obj = match opt {
            Some(t) => {
                if let Some(d) = t.default {
                    d()
                } else {
                    DatType::new_no_param(id)
                }
            }
            None => DatType::new_no_param(id),
        };
        param_obj
    }

    pub fn dat_type_id(&self) -> DatTypeID {
        self.id
    }

    pub fn new_no_param(id: DatTypeID) -> DatType {
        Self { id, param: None }
    }

    pub fn has_no_param(&self) -> bool {
        self.param.is_none()
    }

    pub fn from_info(info: &DTInfo) -> Result<Self, ErrParam> {
        let opt_param = get_fn_param(info.id.to_u32());
        if let Some(fn_param) = opt_param {
            (fn_param.input)(&info.param)
        } else {
            Ok(Self { id: info.id, param: None })
        }
    }

    pub fn new(dat_type_id: DatTypeID, param: Option<DTPKind>) -> Self {
        Self { id: dat_type_id, param }
    }


    pub fn into_info(self) -> DTInfo {
        DTInfo {
            id: self.id,
            param: self.param.map_or(
                Default::default(),
                |p| {
                    p.map(|dt_p| { dt_p.se_to_json().unwrap() })
                }),
        }
    }

    pub fn to_info(&self) -> DTInfo {
        DTInfo {
            id: self.id,
            param: self.param.as_ref().map_or(
                Default::default(),
                |p| {
                    p.map(|dt_p| { dt_p.se_to_json().unwrap() })
                }),
        }
    }

    fn compare(&self, other: &DatType) -> RS<Ordering> {
        let ord = if self.id.has_param() && other.id.has_param() {
            self.id.cmp(&other.id)
        } else {
            let opt_len1 = self.id.fn_send_type_len()(self)
                .map_err(|e| m_error!(EC::TypeErr, "get send type length error", e))?;
            let opt_len2 = other.id.fn_send_type_len()(other)
                .map_err(|e| m_error!(EC::TypeErr, "get send type length error", e))?;
            match (opt_len1, opt_len2) {
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(_), Some(_)) => {
                    self.compare_inner(other)?
                }
                (None, None) => {
                    self.compare_inner(other)?
                }
            }
        };
        Ok(ord)
    }

    fn compare_inner(&self, other: &DatType) -> RS<Ordering> {
        let ord = self.id.cmp(&other.id);
        if ord != Ordering::Equal {
            Ok(ord)
        } else {
            let ord = match (&self.param, &other.param) {
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (Some(k1), Some(k2)) => { k1.cmp_ord(k2)? }
                (None, None) => { Ordering::Equal }
            };
            Ok(ord)
        }
    }
}

impl<'de> Deserialize<'de> for DatType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let info: DTInfo = Deserialize::deserialize(deserializer)?;
        Self::from_info(&info)
            .map_err(|e| {
                Error::custom(format!("error deserializing param object: {:?}", e))
            })
    }
}

impl Serialize for DatType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let info = self.to_info();
        serializer.serialize_some(&info)
    }
}


impl Order for DatType {
    fn cmp_ord(&self, other: &Self) -> RS<Ordering> {
        self.compare(other)
    }
}

macro_rules! impl_dat_type_methods {
    ($((
        $inner_type:ty,
        $variant_upper:ident,
        $variant_lower:ident
    )),+ $(,)?) => {
        $(
            paste! {
                impl DatType {
                    #[doc = "Constructor for type `"]
                    #[doc = stringify!($inner_type)]
                    #[doc = "`"]
                    pub fn [<from_ $variant_lower>](value: $inner_type) -> Self {
                        Self::new(DatTypeID::$variant_upper, Some(DTPKind::$variant_upper(Box::new(value))))
                    }

                    #[doc = "Get reference to internal type`"]
                    #[doc = stringify!($inner_type)]
                    #[doc = "` value"]
                    pub fn [<as_ $variant_lower _param>](&self) -> Option<&$inner_type> {
                        match &self.param {
                            Some(p) => {
                                match p {
                                    DTPKind::$variant_upper(v) => { Some(v.as_ref()) },
                                    _ => { None }
                                }
                            }
                            None => { None }
                        }
                    }

                    #[doc = "Expect get reference to internal `"]
                    #[doc = stringify!($inner_type)]
                    #[doc = "` value"]
                    pub fn [<expect_ $variant_lower _param>](&self) -> &$inner_type {
                        self.[<as_ $variant_lower _param>]().unwrap()
                    }
                }
            }
        )+
    };
}

impl Default for DatType {
    fn default() -> Self {
        DatType::new(DatTypeID::I32, None)
    }
}

impl_dat_type_methods! {
    (DTPString, String, string),
    (DTPObject, Object, object),
    (DTPArray, Array, array),
}