use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Copy, Clone)]
enum State {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    states: Vec<State>,
    groups: Vec<usize>,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.states.iter() {
            match s {
                State::Operational => f.write_str(".")?,
                State::Damaged => f.write_str("#")?,
                State::Unknown => f.write_str("?")?,
            }
        }
        f.write_str(" ")?;
        f.write_fmt(format_args!("{:?}", self.groups))?;
        Ok(())
    }
}

pub fn main(input: &str) -> (String, String) {
    let mut records = Vec::<Record>::new();
    for l in input.lines() {
        let mut n = l.split_whitespace();
        let ss = n.next().unwrap();
        let cr = n.next().unwrap();
        assert!(n.next().is_none());
        let states = ss
            .chars()
            .map(|c| match c {
                '#' => State::Damaged,
                '.' => State::Operational,
                '?' => State::Unknown,
                other => panic!("{other}"),
            })
            .collect::<Vec<_>>();
        let groups = cr
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        records.push(Record { states, groups });
    }

    let options_count_folded = |r: &Record, c: usize| -> usize {
        let mut states = r.states.clone();
        let mut groups = r.groups.clone();
        for _ in 0..c {
            states.push(State::Unknown);
            states.extend(&r.states);
            groups.extend(&r.groups);
        }

        let states = states;
        let groups = groups;

        fn calc(
            states: &Vec<State>,
            groups: &Vec<usize>,
            state_idx: usize,
            group_idx: usize,
            memo: &mut HashMap<(usize, usize), usize>,
        ) -> usize {
            let cur_group = groups[group_idx];

            let mut count = 0usize;
            for i in state_idx..states.len() - (cur_group - 1) {
                // If the previous state is damaged, it's not possible to fit this group in here.
                // We've gone too far.
                if let Some(State::Damaged) = states.get(i.wrapping_sub(1)) {
                    break;
                }

                // Can all the required states for the group be made damaged here?
                if !states[i..i + cur_group]
                    .iter()
                    .all(|s| !matches!(s, State::Operational))
                {
                    continue;
                }

                // Is the next state not damaged?
                if let Some(State::Damaged) = states.get(i + cur_group) {
                    continue;
                }

                let next_group = group_idx + 1;
                let next_state = i + cur_group + 1;

                if next_group < groups.len() {
                    // If there are more groups, either get the memoized result or calculate a new result.
                    match memo.get(&(next_state, next_group)) {
                        Some(v) => count += *v,
                        None => {
                            let v = calc(states, groups, next_state, next_group, memo);
                            memo.insert((next_state, next_group), v);
                            count += v;
                        }
                    }
                } else {
                    // Make sure there aren't any more future damaged states.
                    let extra_damaged = if next_state >= states.len() {
                        0
                    } else {
                        states[next_state..]
                            .iter()
                            .filter(|s| matches!(s, State::Damaged))
                            .count()
                    };
                    if extra_damaged == 0 {
                        count += 1;
                    }
                }
            }
            count
        }

        let mut memo = HashMap::<(usize, usize), usize>::new();
        let count = calc(&states, &groups, 0, 0, &mut memo);
        count
    };

    let folded = records
        .iter()
        .map(|r| options_count_folded(r, 0))
        .collect::<Vec<_>>();
    let folded_sum = folded.iter().sum::<usize>();

    let unfolded_4 = records
        .iter()
        .map(|r| options_count_folded(r, 4))
        .collect::<Vec<_>>();
    let unfolded_4_sum = unfolded_4.iter().sum::<usize>();

    (format!("{folded_sum}"), format!("{unfolded_4_sum}"))
}
