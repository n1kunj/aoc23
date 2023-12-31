mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub const DAYS: &[(&'static str, fn(&str) -> (String, String))] = &[
    ("day1", day1::main),
    ("day2", day2::main),
    ("day3", day3::main),
    ("day4", day4::main),
    ("day5", day5::main),
    ("day6", day6::main),
    ("day7", day7::main),
    ("day8", day8::main),
    ("day9", day9::main),
    ("day10", day10::main),
    ("day11", day11::main),
    ("day12", day12::main),
    ("day13", day13::main),
    ("day14", day14::main),
    ("day15", day15::main),
    ("day16", day16::main),
    ("day17", day17::main),
    ("day18", day18::main),
    ("day19", day19::main),
    ("day20", day20::main),
    ("day21", day21::main),
    ("day22", day22::main),
    ("day23", day23::main),
    ("day24", day24::main),
    ("day25", day25::main),
];
