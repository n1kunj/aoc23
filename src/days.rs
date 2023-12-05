mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub const DAYS: &[(&'static str, fn(&str) -> (String, String))] = &[
    ("day1", day1::main),
    ("day2", day2::main),
    ("day3", day3::main),
    ("day4", day4::main),
    ("day5", day5::main),
    ("day6", day6::main),
    ("day7", day7::main),
];
