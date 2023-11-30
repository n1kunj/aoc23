const TOKENS: &[(&'static str, u32)] = &[
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

pub fn main(input: &str) -> (String, String) {
    let first: Option<u32> = || -> Option<u32> {
        let mut cal_vals = Vec::<u32>::new();

        for line in input.lines() {
            let mut first_digit: Option<u32> = None;
            let mut last_digit: Option<u32> = None;
            for c in line.chars() {
                match c.to_digit(10) {
                    Some(c) => {
                        if first_digit.is_none() {
                            first_digit = Some(c);
                        }
                        last_digit = Some(c)
                    }
                    None => {}
                }
            }
            let concat: String = format!("{}{}", first_digit?, last_digit?);
            cal_vals.push(concat.parse::<u32>().ok()?);
        }
        Some(cal_vals.iter().sum())
    }();

    let second: Option<u32> = || -> Option<u32> {
        let mut cal_vals = Vec::<u32>::new();

        for line in input.lines() {
            let mut first_digit: Option<(usize, u32)> = None;
            let mut last_digit: Option<(usize, u32)> = None;
            for token in TOKENS {
                match line.find(token.0) {
                    Some(idx) => match first_digit {
                        Some((old_idx, _)) => {
                            if idx < old_idx {
                                first_digit = Some((idx, token.1));
                            }
                        }
                        None => {
                            first_digit = Some((idx, token.1));
                        }
                    },
                    None => {}
                }
                match line.rfind(token.0) {
                    Some(idx) => match last_digit {
                        Some((old_idx, _)) => {
                            if idx > old_idx {
                                last_digit = Some((idx, token.1));
                            }
                        }
                        None => {
                            last_digit = Some((idx, token.1));
                        }
                    },
                    None => {}
                }
            }
            let concat: String = format!("{}{}", first_digit?.1, last_digit?.1);
            cal_vals.push(concat.parse::<u32>().ok()?);
        }
        Some(cal_vals.iter().sum())
    }();

    (format!("{first:?}"), format!("{second:?}"))
}
