pub use crate::{
    days::DAYS,
    error::Error,
    get_input, run_day, Day,
    Part::{self, *},
    Reuse,
};

pub use anyhow::{anyhow, bail, ensure, Context, Result};

pub type IResult<'a, T> = nom::IResult<&'a str, T>;
