pub type Days = &'static [Day];

#[derive(Clone, Copy)]
pub struct Day {
    pub day: u8,
    pub examples: &'static [Example],
}

#[derive(Clone, Copy)]
pub struct Example {
    pub example: usize,
    pub parts: &'static [Part],
}

pub struct Part {
    pub part: crate::Part,
    pub expected: &'static str,
}
