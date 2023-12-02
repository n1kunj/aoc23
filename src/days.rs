mod day1;
mod day2;

pub const DAYS: &[(&'static str, fn(&str) -> (String, String))] = &[
    ("day1", day1::main),
    ("day2", day2::main),
];