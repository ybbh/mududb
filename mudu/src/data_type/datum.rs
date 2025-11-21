use crate::common::result::RS;
use crate::data_type::array::new_array_type;
use crate::data_type::dat_binary::DatBinary;
use crate::data_type::dat_textual::DatTextual;
use crate::data_type::dat_type::DatType;
use crate::data_type::dat_type_id::DatTypeID;
use crate::data_type::dat_value::DatValue;
use crate::data_type::dt_of_datum::DtOfDatum;
use crate::data_type::dvi_array::DVIArray;
use crate::error::ec::EC;
use crate::m_error;
use lazy_static::lazy_static;
use paste::paste;
use std::any::Any;
use std::clone::Clone;
use std::fmt;

pub trait Datum: DatumDyn + Clone + 'static {
    fn dat_type() -> &'static DatType;

    fn from_binary(binary: &[u8]) -> RS<Self>;

    fn from_value(value: &DatValue) -> RS<Self>;

    fn from_textual(textual: &str) -> RS<Self>;
}

pub trait DatumDyn: fmt::Debug + Send + Sync + Any {
    fn dat_type_id(&self) -> RS<DatTypeID>;

    fn to_binary(&self, dat_type: &DatType) -> RS<DatBinary>;

    fn to_textual(&self, dat_type: &DatType) -> RS<DatTextual>;

    fn to_value(&self, dat_type: &DatType) -> RS<DatValue>;

    fn clone_boxed(&self) -> Box<dyn DatumDyn>;
}

pub trait AsDatumDynRef {
    fn as_datum_dyn_ref(&self) -> &dyn DatumDyn;
}


fn vec_to_dat_value<D: Datum>(vec: &Vec<D>) -> RS<DatValue> {
    let mut vec_dat_mem = Vec::new();
    for d in vec {
        let internal = d.to_value(D::dat_type())?;
        vec_dat_mem.push(internal);
    }
    let array = DVIArray::new(vec_dat_mem);
    Ok(DatValue::from_array(array))
}

impl<D: Datum> DatumDyn for Vec<D> {
    fn dat_type_id(&self) -> RS<DatTypeID> {
        Ok(DatTypeID::Array)
    }

    fn to_binary(&self, dat_type: &DatType) -> RS<DatBinary> {
        if dat_type.dat_type_id() != DatTypeID::Array {
            return Err(m_error!(EC::TypeErr));
        }
        let dat_mem = vec_to_dat_value(self)?;
        Ok(dat_type.dat_type_id().fn_send()(&dat_mem, dat_type)
               .map_err(|e| m_error!(EC::TypeBaseErr, "convert data format error", e))?,
        )
    }

    fn to_textual(&self, dat_type: &DatType) -> RS<DatTextual> {
        if dat_type.dat_type_id() != DatTypeID::Array {
            return Err(m_error!(EC::TypeErr));
        }
        let dat_mem = vec_to_dat_value(self)?;
        Ok(dat_type.dat_type_id().fn_output()(&dat_mem, dat_type)
               .map_err(|e| m_error!(EC::TypeBaseErr, "convert data format error", e))?,
        )
    }

    fn to_value(&self, dat_type: &DatType) -> RS<DatValue> {
        if dat_type.dat_type_id() != DatTypeID::Array {
            return Err(m_error!(EC::TypeErr));
        }
        let dat_mem = vec_to_dat_value(self)?;
        Ok(dat_mem)
    }

    fn clone_boxed(&self) -> Box<dyn DatumDyn> {
        Box::new(self.clone())
    }
}


impl AsDatumDynRef for Box<dyn DatumDyn> {
    fn as_datum_dyn_ref(&self) -> &dyn DatumDyn {
        self.as_ref()
    }
}

impl<U: AsDatumDynRef + ?Sized> AsDatumDynRef for &U {
    fn as_datum_dyn_ref(&self) -> &dyn DatumDyn {
        (*self).as_datum_dyn_ref()
    }
}

impl<'a, U: AsDatumDynRef> AsDatumDynRef for &'a [U] {
    fn as_datum_dyn_ref(&self) -> &dyn DatumDyn {
        if self.is_empty() {
            panic!("Empty slice");
        }
        self[0].as_datum_dyn_ref()
    }
}

impl<T: AsDatumDynRef> AsDatumDynRef for Vec<T> {
    fn as_datum_dyn_ref(&self) -> &dyn DatumDyn {
        if self.is_empty() {
            panic!("Empty vector");
        }
        self[0].as_datum_dyn_ref()
    }
}

impl<T: AsDatumDynRef, const N: usize> AsDatumDynRef for [T; N] {
    fn as_datum_dyn_ref(&self) -> &dyn DatumDyn {
        if self.is_empty() {
            panic!("Empty array");
        }
        self[0].as_datum_dyn_ref()
    }
}


pub fn binary_to_typed<T: Datum, S: AsRef<str>>(
    data: &[u8],
    _type_str: S,
) -> RS<T> {
    T::from_binary(data)
}

pub fn binary_from_typed<T: Datum, S: AsRef<str>>(
    t: &T,
    _type_str: S,
) -> RS<Vec<u8>> {
    let dat_bin = t.to_binary(T::dat_type())?;
    Ok(dat_bin.into())
}


impl<D: Datum> Datum for Vec<D> {
    fn dat_type() -> &'static DatType {
        lazy_static! {
            static ref DT_OF_DATUM: DtOfDatum = DtOfDatum::new();
        }
        DT_OF_DATUM.ty_of_datum::<Self, _>(|| {
            new_array_type(D::dat_type().clone())
        })
    }

    fn from_binary(binary: &[u8]) -> RS<Self> {
        let dat_type = Self::dat_type();
        let (dat_mem, _) = dat_type.dat_type_id().fn_recv()(binary, dat_type)
            .map_err(|e| m_error!(EC::TypeErr, "error when convert binary to array type", e))?;
        Self::from_value(&dat_mem)
    }

    fn from_value(mem: &DatValue) -> RS<Self> {
        let array = mem.expect_array();
        let mut vec_d = Vec::with_capacity(array.array().len());
        for dat in array.array() {
            let d = D::from_value(dat)?;
            vec_d.push(d);
        }
        Ok(vec_d)
    }

    fn from_textual(textual: &str) -> RS<Self> {
        let dat_type = Self::dat_type();
        let dat_value = dat_type.dat_type_id().fn_input()(textual, dat_type)
            .map_err(|e| m_error!(EC::TypeErr, "error when convert textual to array type", e))?;
        Self::from_value(&dat_value)
    }
}

macro_rules! impl_datum_trait {
    ($(($variant_upper:ident, $variant_lower:ident, $datum_type:ty)),+ $(,)?) => {
        $(
            impl Datum for $datum_type {
                paste! {
                    fn dat_type() -> &'static DatType {
                        lazy_static!{
                            static ref DAT_TYPE:DatType = DatType::default_for(DatTypeID::$variant_upper);
                        }
                        &DAT_TYPE
                    }

                    fn from_binary(binary: &[u8]) -> RS<Self> {
                        let dat_type = Self::dat_type();
                        let (dat_mem, _) = dat_type.dat_type_id().fn_recv()(&binary, dat_type)
                            .map_err(|e|{
                                m_error!(EC::TypeErr, "error when convert binary to memory type", e)
                            })?;
                        let value = dat_mem.[<expect_ $variant_lower>]();
                        Ok(value.clone())
                    }

                    fn from_value(dat_mem: &DatValue) -> RS<Self> {
                        let value = dat_mem.[<expect_ $variant_lower>]();
                        Ok(value.clone())
                    }

                    fn from_textual(textual: &str) -> RS<Self> {
                        let dat_type = Self::dat_type();
                        let dat_value = dat_type.dat_type_id().fn_input()(textual, dat_type)
                            .map_err(|e| m_error!(EC::TypeErr, "error when convert textual to array type", e))?;
                        Self::from_value(&dat_value)
                    }
                }
            }


            impl DatumDyn for $datum_type {
                paste! {
                    fn dat_type_id(&self) -> RS<DatTypeID> {
                        Ok(DatTypeID::$variant_upper)
                    }

                    fn to_binary(&self, dat_type: &DatType) -> RS<DatBinary> {
                        if dat_type.dat_type_id() != DatTypeID::$variant_upper {
                            return Err(m_error!(EC::TypeErr));
                        }
                        Ok(dat_type.dat_type_id().fn_send()(&DatValue::[<from_ $variant_lower>](self.clone()), dat_type)
                             .map_err(|e| m_error!(EC::TypeBaseErr, "convert data format error", e))?,
                        )
                    }

                    fn to_textual(&self, dat_type: &DatType) -> RS<DatTextual> {
                        if dat_type.dat_type_id() != DatTypeID::$variant_upper {
                            return Err(m_error!(EC::TypeErr));
                        }
                        Ok(dat_type.dat_type_id().fn_output()(&DatValue::[<from_ $variant_lower>](self.clone()), dat_type)
                             .map_err(|e| m_error!(EC::TypeBaseErr, "convert data format error", e))?,
                        )
                    }

                    fn to_value(&self, dat_type: &DatType) -> RS<DatValue> {
                        if dat_type.dat_type_id() != DatTypeID::$variant_upper {
                            return Err(m_error!(EC::TypeErr));
                        }
                        Ok(DatValue::[<from_ $variant_lower>](self.clone()))
                    }

                    fn clone_boxed(&self) -> Box<dyn DatumDyn> {
                        Box::new(self.clone())
                    }
                }
            }
        )+
    };
}


impl_datum_trait!(
    (I32, i32, i32),
    (I64, i64, i64),
    (F32, f32, f32),
    (F64, f64, f64),
    (String, string, String)
);



