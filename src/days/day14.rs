use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Ground {
    Round,
    Empty,
    Cube,
}

pub fn main(input: &str) -> (String, String) {
    let mut rows = Vec::<Vec<Ground>>::new();
    for l in input.lines() {
        rows.push(
            l.chars()
                .map(|c| match c {
                    'O' => Ground::Round,
                    '.' => Ground::Empty,
                    '#' => Ground::Cube,
                    other => panic!("{other}"),
                })
                .collect::<Vec<_>>(),
        );
    }
    let rows = rows;

    #[allow(dead_code)]
    fn pr(rows: &Vec<Vec<Ground>>) {
        for row in rows {
            for g in row {
                let c = match g {
                    Ground::Round => 'O',
                    Ground::Empty => '.',
                    Ground::Cube => '#',
                };
                print!("{c}");
            }
            println!();
        }
        println!();
    }

    fn north(rows: &mut Vec<Vec<Ground>>) -> bool {
        let mut any_moved = false;
        for r_idx in 1..rows.len() {
            for c_idx in 0..rows[0].len() {
                let dst = &rows[r_idx - 1][c_idx];
                let src = &rows[r_idx][c_idx];

                if matches!(dst, Ground::Empty) && matches!(src, Ground::Round) {
                    rows[r_idx - 1][c_idx] = Ground::Round;
                    rows[r_idx][c_idx] = Ground::Empty;
                    any_moved = true;
                }
            }
        }
        any_moved
    }

    fn east(rows: &mut Vec<Vec<Ground>>) -> bool {
        let mut any_moved = false;
        for c_idx in 0..(rows.len() - 1) {
            for r_idx in 0..rows.len() {
                let dst = &rows[r_idx][c_idx + 1];
                let src = &rows[r_idx][c_idx];

                if matches!(dst, Ground::Empty) && matches!(src, Ground::Round) {
                    rows[r_idx][c_idx + 1] = Ground::Round;
                    rows[r_idx][c_idx] = Ground::Empty;
                    any_moved = true;
                }
            }
        }
        any_moved
    }

    fn south(rows: &mut Vec<Vec<Ground>>) -> bool {
        let mut any_moved = false;
        for r_idx in (0..(rows.len() - 1)).rev() {
            for c_idx in 0..rows[0].len() {
                let dst = &rows[r_idx + 1][c_idx];
                let src = &rows[r_idx][c_idx];

                if matches!(dst, Ground::Empty) && matches!(src, Ground::Round) {
                    rows[r_idx + 1][c_idx] = Ground::Round;
                    rows[r_idx][c_idx] = Ground::Empty;
                    any_moved = true;
                }
            }
        }
        any_moved
    }

    fn west(rows: &mut Vec<Vec<Ground>>) -> bool {
        let mut any_moved = false;
        for c_idx in (1..rows[0].len()).rev() {
            for r_idx in 0..rows.len() {
                let dst = &rows[r_idx][c_idx - 1];
                let src = &rows[r_idx][c_idx];

                if matches!(dst, Ground::Empty) && matches!(src, Ground::Round) {
                    rows[r_idx][c_idx - 1] = Ground::Round;
                    rows[r_idx][c_idx] = Ground::Empty;
                    any_moved = true;
                }
            }
        }
        any_moved
    }

    fn roll<F>(op: F, rows: &mut Vec<Vec<Ground>>)
    where
        F: Fn(&mut Vec<Vec<Ground>>) -> bool,
    {
        loop {
            let any_moved = op(rows);
            if !any_moved {
                break;
            }
        }
    }

    fn total_load(rows: &Vec<Vec<Ground>>) -> usize {
        let round_count_per_row = rows
            .iter()
            .map(|r| r.iter().filter(|g| matches!(g, Ground::Round)).count())
            .collect::<Vec<_>>();

        let load_per_row = round_count_per_row
            .iter()
            .enumerate()
            .map(|(i, c)| c * (rows.len() - i))
            .collect::<Vec<_>>();
        load_per_row.iter().sum::<usize>()
    }

    let mut north_rows = rows.clone();
    roll(north, &mut north_rows);
    let north_total_load = total_load(&north_rows);

    let mut cycle_rows = rows.clone();

    let mut row_to_idx = HashMap::<Vec<Vec<Ground>>, usize>::new();
    row_to_idx.insert(rows.clone(), 0);

    const ITS: usize = 1000000000;
    let mut i = 0;
    loop {
        roll(north, &mut cycle_rows);
        roll(west, &mut cycle_rows);
        roll(south, &mut cycle_rows);
        roll(east, &mut cycle_rows);

        i += 1;
        if let Some(v) = row_to_idx.get(&cycle_rows) {
            let dist = i - v;
            let remaining = ITS - i;
            i += dist * (remaining / dist);
        } else {
            row_to_idx.insert(cycle_rows.clone(), i);
        }
        if i == ITS {
            break;
        }
    }

    let cycle_total_load = total_load(&cycle_rows);

    (format!("{north_total_load}"), format!("{cycle_total_load}"))
}
