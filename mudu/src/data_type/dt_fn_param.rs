pub use crate::data_type::dat_type::DatType;
use std::fmt;

#[derive(Debug)]
pub enum ErrParam {
    ParamParseError(String),
}
#[derive(Clone, Debug)]
pub struct FnParam {
    pub input: FnParamIn,
    pub default: Option<FnParamDefault>,
}

pub type FnParamIn = fn(params: &str) -> Result<DatType, ErrParam>;

pub type FnParamDefault = fn() -> DatType;

impl fmt::Display for FnParam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}
