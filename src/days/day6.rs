use std::fmt::Write;

struct Race {
    t: usize,
    d: usize,
}

pub fn main(input: &str) -> (String, String) {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .replace("Time:", "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .replace("Distance:", "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    assert!(times.len() == distances.len());

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race { t: *t, d: *d })
        .collect::<Vec<_>>();

    let calc_win_count = |r: &Race| -> usize {
        let mut win_count = 0usize;
        for hold_time in 0..=r.t {
            let rest_time = r.t - hold_time;
            let distance = rest_time * hold_time;
            let is_win = distance > r.d;
            if is_win {
                win_count += 1;
            }
        }
        win_count
    };

    let win_counts = races.iter().map(calc_win_count).collect::<Vec<_>>();
    let win_counts_mult = win_counts.iter().fold(1usize, |a, e| a * e);

    let mut t_concat = String::new();
    let mut d_concat = String::new();
    for r in races.iter() {
        write!(&mut t_concat, "{}", r.t).unwrap();
        write!(&mut d_concat, "{}", r.d).unwrap();
    }

    let race_cat = Race {
        t: t_concat.parse::<usize>().unwrap(),
        d: d_concat.parse::<usize>().unwrap(),
    };
    let cat_win_count = calc_win_count(&race_cat);

    (format!("{win_counts_mult}"), format!("{cat_win_count}"))
}
