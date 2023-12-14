use std::iter::once;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Ground {
    Ash,
    Rock,
}

#[derive(Clone, Debug)]
struct Pattern {
    rows: Vec<Vec<Ground>>,
    cols: Vec<Vec<Ground>>,
}

impl Pattern {
    fn new(rows: Vec<Vec<Ground>>) -> Pattern {
        let cols = (0..rows[0].len())
            .map(|c| rows.iter().map(|r| r[c]).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Pattern { rows, cols }
    }
}

pub fn main(input: &str) -> (String, String) {
    let mut patterns = Vec::<Pattern>::new();

    let mut rows = Vec::<Vec<Ground>>::new();
    for l in input.lines().chain(once("")) {
        if l.is_empty() {
            if !rows.is_empty() {
                patterns.push(Pattern::new(rows.clone()));
                rows.clear();
            }
        } else {
            rows.push(
                l.chars()
                    .map(|c| match c {
                        '#' => Ground::Rock,
                        '.' => Ground::Ash,
                        other => panic!("{other}"),
                    })
                    .collect::<Vec<_>>(),
            );
        }
    }
    assert!(rows.is_empty());

    fn pre_refl_idx(gs: &Vec<Vec<Ground>>, ignore: Option<usize>) -> Option<usize> {
        for r in 0..(gs.len() - 1) {
            if ignore.is_some_and(|ignore| ignore == r) {
                continue;
            }
            let mut off = 0usize;
            loop {
                let pre_ri = r.wrapping_sub(off);
                let post_ri = r + 1 + off;
                let pre_r = gs.get(pre_ri);
                let post_r = gs.get(post_ri);

                match (pre_r, post_r) {
                    (Some(pre), Some(post)) => {
                        if pre != post {
                            break;
                        }
                    }
                    (_, _) => return Some(r),
                }

                off += 1;
            }
        }

        None
    }

    fn score(p: &Pattern) -> usize {
        let row_pre_refl_idx = pre_refl_idx(&p.rows, None);
        let col_pre_refl_idx = pre_refl_idx(&p.cols, None);

        match (row_pre_refl_idx, col_pre_refl_idx) {
            (None, None) => panic!("No matches"),
            (None, Some(c)) => c + 1,
            (Some(r), None) => (r + 1) * 100,
            (Some(_), Some(_)) => panic!("Both matches"),
        }
    }

    let scores: Vec<usize> = patterns.iter().map(|p| score(p)).collect::<Vec<_>>();
    let summary = scores.iter().sum::<usize>();

    fn score2(p: &Pattern) -> usize {
        let og_row_pre_refl_idx = pre_refl_idx(&p.rows, None);
        let og_col_pre_refl_idx = pre_refl_idx(&p.cols, None);

        for row in 0..p.rows.len() {
            for col in 0..p.rows[0].len() {
                let mut rows = p.rows.clone();
                rows[row][col] = match rows[row][col] {
                    Ground::Ash => Ground::Rock,
                    Ground::Rock => Ground::Ash,
                };
                let p2 = Pattern::new(rows);

                let row_pre_refl_idx = pre_refl_idx(&p2.rows, og_row_pre_refl_idx);
                let col_pre_refl_idx = pre_refl_idx(&p2.cols, og_col_pre_refl_idx);

                match (row_pre_refl_idx, col_pre_refl_idx) {
                    (None, None) => (),
                    (None, Some(c)) => {
                        return c + 1;
                    }
                    (Some(r), None) => {
                        return (r + 1) * 100;
                    }
                    (Some(_), Some(_)) => panic!("Both matches!"),
                }
            }
        }
        panic!("No matches!");
    }

    let scores2: Vec<usize> = patterns.iter().map(|p| score2(p)).collect::<Vec<_>>();
    let summary2 = scores2.iter().sum::<usize>();

    (format!("{summary}"), format!("{summary2}"))
}
