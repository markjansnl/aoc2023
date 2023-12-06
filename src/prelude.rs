pub use crate::{
    bench_day,
    days::DAYS,
    def,
    error::Error,
    get_input, reuse_parsed, run_day, Day,
    Part::{self, *},
    Reuse,
};

pub use anyhow::{anyhow, bail, ensure, Context, Result};

pub type IResult<'a, T> = nom::IResult<&'a str, T>;
