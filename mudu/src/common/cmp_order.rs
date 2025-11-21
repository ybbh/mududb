use crate::error::err::MError;
use std::cmp::Ordering;

pub trait Order {
    type Error = MError;

    fn cmp_ord(&self, other: &Self) -> Result<Ordering, Self::Error>;
}
