use crate::common::result::RS;
use crate::data_type::dat_type_id::DatTypeID;
use crate::data_type::datum::Datum;
use crate::data_type::dt_fn_param::DatType;
use crate::data_type::dvi_array::DVIArray;
use crate::data_type::dvi_object::DVIObject;
use paste::paste;
use std::fmt::Debug;
use std::hint;

/// A memory-efficient representation of data that can hold various primitive types
/// or complex types (arrays, objects) in a unified enum container.
#[derive(Clone, Debug)]
pub struct DatValue {
    inner: ValueKind,
}

// Mark as thread-safe since all variants are either primitive types or boxed types
unsafe impl Send for DatValue {}
unsafe impl Sync for DatValue {}

impl AsRef<DatValue> for DatValue {
    fn as_ref(&self) -> &DatValue {
        self
    }
}

/// Internal memory representation supporting various data types
/// Uses Box for heap allocation of complex types to avoid large enum variants
enum ValueKind {
    F32(f32),
    F64(f64),
    I32(i32),
    I64(i64),
    String(Box<String>),
    Object(Box<DVIObject>),
    Array(Box<DVIArray>),
}

macro_rules! impl_dat_mem_methods {
    ($((
        $is_boxed:ident,
        $inner_type:ty,
        $dvi_type:ty,
        $variant_upper:ident,
        $variant_lower:ident
    )),+ $(,)?) => {
        $(
            impl_dat_mem_methods!(
                @impl_variant
                    $is_boxed,
                    $inner_type,
                    $dvi_type,
                    $variant_upper,
                    $variant_lower
            );
        )+

        // Automatically generates debug arms for all enum variant
        impl std::fmt::Debug for ValueKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        ValueKind::$variant_upper(value) => {
                            write!(f, "{}({:?})", stringify!($variant_upper), value)
                        }
                    )+
                }
            }
        }

        // Automatically generates clone arms for all enum variant
        impl Clone for ValueKind {
            fn clone(&self) -> Self {
                match self {
                    $(
                        ValueKind::$variant_upper(value) => {
                            Self::$variant_upper(value.clone())
                        }
                    )+
                }
            }
        }
    };

    // Special handling for Box<T> types
    (@impl_variant true, $inner_type:ty , $dvi_type:ty, $variant_upper:ident, $variant_lower:ident) => {
        paste! {
            impl DatValue {
                #[doc = "Constructor for type `"]
                #[doc = stringify!($dvi_type)]
                #[doc = "`"]
                pub fn [<from_ $variant_lower>](value: $dvi_type) -> Self {
                    Self { inner: ValueKind::[<from_ $variant_lower>](value) }
                }

                #[doc = "Constructor for boxed type`"]
                #[doc = stringify!($dvi_type)]
                #[doc = "`"]
                pub fn [<from_boxed_ $variant_lower>](value: $inner_type) -> Self {
                    Self { inner: ValueKind::[<from_boxed_ $variant_lower>](value) }
                }

                #[doc = "Get reference to internal type`"]
                #[doc = stringify!($dvi_type)]
                #[doc = "` value"]
                pub fn [<as_ $variant_lower>](&self) -> Option<&$dvi_type> {
                    self.inner.[<as_ $variant_lower>]()
                }

                #[doc = "Get reference to boxed internal`"]
                #[doc = stringify!($dvi_type)]
                #[doc = "` value"]
                pub fn [<as_boxed_ $variant_lower>](&self) -> Option<&$inner_type> {
                    self.inner.[<as_boxed_ $variant_lower>]()
                }

                #[doc = "Expect get reference to internal `"]
                #[doc = stringify!($dvi_type)]
                #[doc = "` value"]
                pub fn [<expect_ $variant_lower>](&self) -> &$dvi_type {
                    self.inner.[<expect_ $variant_lower>]()
                }
            }

            impl ValueKind {
                fn [<from_ $variant_lower>](value: $dvi_type) -> Self {
                    Self::[<from_boxed_ $variant_lower>](Box::new(value))
                }

                fn [<from_boxed_ $variant_lower>](value: $inner_type) -> Self {
                    ValueKind::$variant_upper(value)
                }

                fn [<as_ $variant_lower>](&self) -> Option<&$dvi_type> {
                    if let ValueKind::$variant_upper(v) = self {
                        Some(v.as_ref())
                    } else {
                        None
                    }
                }

                fn [<as_boxed_ $variant_lower>](&self) -> Option<&$inner_type> {
                    if let ValueKind::$variant_upper(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn [<expect_ $variant_lower>](&self) -> &$dvi_type {
                    unsafe {
                        match self {
                            ValueKind::$variant_upper(value) => value.as_ref(),
                            _ => { hint::unreachable_unchecked() }
                        }
                    }
                }
            }
        }
    };

    // Handling for non-boxed types
    (@impl_variant false, $inner_type:ty,  $dvi_type:ty, $variant_upper:ident, $variant_lower:ident) => {
        paste! {
            impl DatValue {
                #[doc = "Constructor for `"]
                #[doc = stringify!($dvi_type)]
                #[doc = "`"]
                pub fn [<from_ $variant_lower>](value: $inner_type) -> Self {
                    Self { inner: ValueKind::[<from_ $variant_lower>](value) }
                }

                #[doc = "Get reference to internal `"]
                #[doc = stringify!($dvi_type)]
                #[doc = "` value"]
                pub fn [<as_ $variant_lower>](&self) -> Option<&$inner_type> {
                    self.inner.[<as_ $variant_lower>]()
                }

                #[doc = "Expect get reference to internal `"]
                #[doc = stringify!($dvi_type)]
                #[doc = "` value"]
                pub fn [<expect_ $variant_lower>](&self) -> &$dvi_type {
                    self.inner.[<expect_ $variant_lower>]()
                }
            }

            impl ValueKind {
                fn [<from_ $variant_lower>](value: $inner_type) -> Self {
                    ValueKind::$variant_upper(value)
                }

                fn [<as_ $variant_lower>](&self) -> Option<&$inner_type> {
                    if let ValueKind::$variant_upper(v) = self {
                        Some(v)
                    } else {
                        None
                    }
                }

                fn [<expect_ $variant_lower>](&self) -> &$dvi_type {
                    unsafe {
                        match self {
                            ValueKind::$variant_upper(value) => value,
                            _ => { hint::unreachable_unchecked() }
                        }
                    }
                }
            }
        }
    };
}


impl DatValue {
    /// Creates a MemDatum from any type implementing Datum trait with type information
    pub fn from_datum<T: Datum>(datum: T, type_obj: &DatType) -> RS<Self> {
        Ok(Self {
            inner: ValueKind::from_datum(datum, type_obj)?
        })
    }

    /// Conversion methods to owned values
    pub fn to_f32(&self) -> f32 {
        self.expect_f32().clone()
    }

    pub fn to_f64(&self) -> f64 {
        self.expect_f64().clone()
    }

    pub fn to_i32(&self) -> i32 {
        self.expect_i32().clone()
    }

    pub fn to_i64(&self) -> i64 {
        self.expect_i64().clone()
    }
}


/// Safe wrapper for unsafe pointer casting between types
/// Assumes the caller guarantees type compatibility
#[inline]
#[allow(unused)]
fn unsafe_cast<S, D>(src: &S) -> &D {
    unsafe { &*(src as *const S as *const D) }
}

impl ValueKind {
    /// Internal method to create ValueKind from Datum with type information
    fn from_datum<T: Datum>(datum: T, type_obj: &DatType) -> RS<Self> {
        Ok(datum.to_value(type_obj)?.inner)
    }


    /// Returns a typed reference by casting based on the Datum type ID
    #[allow(unused)]
    fn to_typed_ref<T: Datum>(&self) -> &T {
        let data_type = T::dat_type();
        match data_type.dat_type_id() {
            DatTypeID::F32 => unsafe_cast(self.expect_f32()),
            DatTypeID::F64 => unsafe_cast(self.expect_f64()),
            DatTypeID::I32 => unsafe_cast(self.expect_i32()),
            DatTypeID::I64 => unsafe_cast(self.expect_i64()),
            DatTypeID::String => unsafe_cast(self.expect_string()),
            DatTypeID::Array => { todo!() },
            DatTypeID::Object => { todo!() },
        }
    }
}

// Mark internal enum as thread-safe since all variants are either primitive or boxed
unsafe impl Send for ValueKind {}
unsafe impl Sync for ValueKind {}


impl_dat_mem_methods! {
    (false, i32, i32, I32, i32),
    (false, i64, i64, I64, i64),
    (false, f32, f32, F32, f32),
    (false, f64, f64, F64, f64),
    (true, Box<String>, String, String, string),
    (true, Box<DVIArray>, DVIArray, Array, array),
    (true, Box<DVIObject>, DVIObject, Object, object),
}

#[cfg(test)]
mod tests {
    use crate::data_type::dat_value::DatValue;

    #[test]
    fn test() {
        let s = "string";
        let mem = DatValue::from_string(s.to_string());
        assert_eq!(mem.as_string(), Some(&s.to_string()));
        assert_eq!(mem.as_boxed_string(), Some(&Box::new(s.to_string())));
        assert_eq!(mem.expect_string(), &s.to_string());
        assert!(mem.as_i32().is_none());

        let i = 10;
        let mem = DatValue::from_i32(i);
        assert_eq!(mem.as_i32(), Some(&i));
        assert_eq!(mem.expect_i32(), &i);
        assert!(mem.as_string().is_none());
    }
}