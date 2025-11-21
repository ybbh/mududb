use crate::common::result::RS;
use crate::data_type::dat_type::DatType;
use crate::data_type::datum::Datum;
use crate::tuple::datum_desc::DatumDesc;
use crate::tuple::enumerable_datum::EnumerableDatum;
use crate::tuple::tuple_field_desc::TupleFieldDesc;

// Defines conversion methods between Rust tuples and binary data with description information.
/**
For a tuple (i32, String)
```
    use crate::mudu::tuple::enumerable_datum::EnumerableDatum;
    use crate::mudu::tuple::rs_tuple_datum::RsTupleDatum;

    let data = (42, "hello".to_string());
    let desc = <(i32, String)>::tuple_desc_static(&["field_1".to_string(), "field_2".to_string()]);
    let binary = data.to_binary(desc.fields()).unwrap();
    let decoded = <(i32, String)>::from_binary(&binary, desc.fields()).unwrap();
```
**/

pub trait RsTupleDatum: EnumerableDatum + Sized + 'static {
    fn from_binary(vec_bin: &Vec<Vec<u8>>, desc: &[DatumDesc]) -> RS<Self>;
    fn tuple_desc_static(field_name: &[String]) -> TupleFieldDesc;
}

fn datum_from_binary<T: Datum>(slice: &[u8], _desc: &DatumDesc) -> RS<T> {
    T::from_binary(slice)
}

fn datum_to_binary<T: Datum>(t: &T, desc: &DatumDesc) -> RS<Vec<u8>> {
    let binary = t.to_binary(desc.dat_type())?;
    Ok(binary.into())
}

fn to_tuple_desc(fields: Vec<(String, DatType)>) -> TupleFieldDesc {
    let desc: Vec<_> = fields
        .into_iter()
        .map(|(name, ty)| {
            let desc = DatumDesc::new(name, ty);
            desc
        })
        .collect();
    TupleFieldDesc::new(desc)
}

fn build_tuple_desc(field_name: &[String], field_ty: Vec<DatType>) -> TupleFieldDesc {
    let fields: Vec<(String, DatType)> = if field_ty.len() == field_name.len() {
        field_ty.into_iter().enumerate().map(|(i, ty)| {
            (field_name[i].clone(), ty)
        }).collect()
    } else {
        field_ty.into_iter().enumerate().map(|(i, ty)| {
            (format!("field_{}", i), ty)
        }).collect()
    };
    to_tuple_desc(fields)
}


impl<T> EnumerableDatum for T
where
    T: Datum,
{
    fn to_binary(&self, desc: &[DatumDesc]) -> RS<Vec<Vec<u8>>> {
        if desc.len() != 1 {
            panic!("single type expects exactly one DatumDesc");
        }
        let binary = datum_to_binary(self, &desc[0])?;
        Ok(vec![binary])
    }

    fn tuple_desc(&self, field_name: &[String]) -> RS<TupleFieldDesc> {
        Ok(Self::tuple_desc_static(field_name))
    }
}

impl<T> RsTupleDatum for T
where
    T: Datum,
{
    fn from_binary(vec_bin: &Vec<Vec<u8>>, desc: &[DatumDesc]) -> RS<T> {
        if vec_bin.len() != 1 || desc.len() != 1 {
            panic!("single type expects exactly one binary and one DatumDesc");
        }
        datum_from_binary::<T>(&vec_bin[0], &desc[0])
    }

    fn tuple_desc_static(field_name: &[String]) -> TupleFieldDesc {
        let ty = T::dat_type().clone();
        let name = if field_name.len() == 1 {
            field_name[0].clone()
        } else {
            Default::default()
        };
        to_tuple_desc(vec![(name, ty)])
    }
}

macro_rules! impl_rs_tuple_datum {
    // basic: empty tuple
    () => {
        impl EnumerableDatum for () {
            fn to_binary(&self, _desc: &[DatumDesc]) -> RS<Vec<Vec<u8>>> {
                Ok(vec![])
            }

            fn tuple_desc(&self, _field_name:&[String]) -> RS<TupleFieldDesc> {
                Ok(TupleFieldDesc::new(vec![]))
            }
        }

        impl RsTupleDatum for () {
            fn from_binary(_vec_bin: &Vec<Vec<u8>>, _desc: &[DatumDesc]) -> RS<()> {
                Ok(())
            }

            fn tuple_desc_static(_field_name:&[String]) -> TupleFieldDesc {
                TupleFieldDesc::new(vec![])
            }
        }
    };

    // recursive：handle tuple (T, T..., T)
    ($($T:ident),+) => {
        impl<$($T: Datum),*> EnumerableDatum for ($($T,)*) {
            #[allow(non_snake_case)]
            #[allow(unused_assignments)]
            fn to_binary(&self, desc: &[DatumDesc]) -> RS<Vec<Vec<u8>>> {
                if desc.len() < 1 {
                    panic!("tuple size error");
                }
                let mut vec_binary = Vec::new();
                let ($(ref $T,)*) = *self;
                let mut idx = 0;
                $(
                    vec_binary.push(datum_to_binary($T, &desc[idx])?);
                    idx += 1;
                )*
                Ok(vec_binary)
            }

            fn tuple_desc(&self, field_name:&[String]) -> RS<TupleFieldDesc> {
                Ok(Self::tuple_desc_static(field_name))
            }
        }

        impl<$($T: Datum),*> RsTupleDatum for ($($T,)*) {
            #[allow(non_snake_case)]
            #[allow(unused_assignments)]
            fn from_binary(vec_bin: &Vec<Vec<u8>>, desc: &[DatumDesc]) -> RS<($($T,)*)> {
                if vec_bin.len() != desc.len() {
                    panic!("tuple size error");
                }
                let mut idx = 0;
                $(
                    let $T = datum_from_binary::<$T>(&vec_bin[idx], &desc[idx])?;
                    idx += 1;
                )*
                Ok(($($T,)*))
            }

            fn tuple_desc_static(field_name:&[String]) -> TupleFieldDesc {
                let vec_ty:Vec<DatType> = vec![
                    $(<$T>::dat_type().clone(),)*
                ];
                build_tuple_desc(field_name, vec_ty)
            }
        }
    };
}

impl_rs_tuple_datum!();
impl_rs_tuple_datum!(A);
impl_rs_tuple_datum!(A, B);
impl_rs_tuple_datum!(A, B, C);
impl_rs_tuple_datum!(A, B, C, D);
impl_rs_tuple_datum!(A, B, C, D, E);
impl_rs_tuple_datum!(A, B, C, D, E, F);
impl_rs_tuple_datum!(A, B, C, D, E, F, G);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_rs_tuple_datum!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1,
    E1
);
impl_rs_tuple_datum!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, A1, B1, C1, D1,
    E1, F1
);


#[cfg(test)]
mod tests {
    use crate::tuple::rs_tuple_datum;

    #[test]
    fn test_tuple_datum() {
        println!("{:?}", <i32 as rs_tuple_datum::RsTupleDatum>::tuple_desc_static(
            &["test_field1".to_string()]
        ));
        println!("{:?}", <(i32,) as rs_tuple_datum::RsTupleDatum>::tuple_desc_static(&[]));
        println!("{:?}", <(i32, i64) as rs_tuple_datum::RsTupleDatum>::tuple_desc_static(
            &["f1".to_string(), "f2".to_string()]
        ));
    }
}