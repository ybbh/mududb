use crate::common::cmp_order::Order;
use crate::common::result::RS;
use crate::data_type::dat_type_id::DatTypeID;
use crate::data_type::dt_param::DTPDyn;
use crate::data_type::dtp_array::DTPArray;
use crate::data_type::dtp_object::DTPObject;
use crate::data_type::dtp_string::DTPString;
use paste::paste;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum DTPKind {
    String(Box<DTPString>),
    Object(Box<DTPObject>),
    Array(Box<DTPArray>),
}


impl DTPKind {

}


macro_rules! impl_dtp_kind_methods {
    ($((
        $inner_type:ty,
        $variant_upper:ident,
        $variant_lower:ident
    )),+ $(,)?) => {
        paste! {
            impl DTPKind {
                #[doc = "map inner `"]
                #[doc = "`"]
                pub fn map<U, F>(&self, f: F) -> U
                where
                    F: FnOnce(&dyn DTPDyn) -> U,
                {
                    match self {
                        $(DTPKind::$variant_upper(p) => { f(p.as_ref()) })*
                    }
                }

                pub fn dat_type_id(&self) -> DatTypeID {
                    match self {
                        $(DTPKind::$variant_upper(_) => { DatTypeID::$variant_upper })*
                    }
                }

                pub fn as_dtp_dyn(&self) -> & dyn DTPDyn {
                    match self {
                        $(DTPKind::$variant_upper(p) => { p.as_ref() })*
                    }
                }

                pub fn compare(&self, other: &Self) -> RS<Ordering> {
                    let ord = match (self, other) {
                        $((DTPKind::$variant_upper(l), DTPKind::$variant_upper(r)) => { l.cmp_ord(r)? })*
                        _ => { self.dat_type_id().cmp(&other.dat_type_id()) }
                    };
                    Ok(ord)
                }
            }
        }
        $(
            paste! {
                impl DTPKind {
                    #[doc = "Get reference to internal type`"]
                    #[doc = stringify!($inner_type)]
                    #[doc = "` value"]
                    #[inline]
                    pub fn [<as_ $variant_lower _param>](&self) -> Option<&$inner_type> {
                        match self {
                            DTPKind::$variant_upper(v) => { Some(v.as_ref()) },
                            _ => { None }
                        }
                    }

                    #[doc = "Expect get reference to internal `"]
                    #[doc = stringify!($inner_type)]
                    #[doc = "` value"]
                    #[inline]
                    pub fn [<expect_ $variant_lower _param>](&self) -> &$inner_type {
                        self.[<as_ $variant_lower _param>]().unwrap()
                    }
                }
            }
        )+
    };
}


impl Order for DTPKind {
    fn cmp_ord(&self, other: &Self) -> RS<Ordering> {
        self.compare(other)
    }
}



impl_dtp_kind_methods! {
    (DTPString, String, string),
    (DTPObject, Object, object),
    (DTPArray, Array, array),
}