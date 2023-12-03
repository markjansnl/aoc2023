pub use crate::{
    error::Error,
    Day,
    Part::{self, *},
    Reuse,
};

pub use anyhow::{anyhow, bail, ensure, Context, Result};

pub type IResult<'a, T> = nom::IResult<&'a str, T>;
