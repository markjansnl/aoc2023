pub use crate::{
    bench_day_part,
    days::DAYS,
    error::Error,
    get_input, reuse_parsed, bench_parse_day, run_day, Day,
    Part::{self, *},
    Reuse,
    def
};

pub use anyhow::{anyhow, bail, ensure, Context, Result};

pub type IResult<'a, T> = nom::IResult<&'a str, T>;
