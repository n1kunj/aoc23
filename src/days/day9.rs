pub fn main(input: &str) -> (String, String) {
    let mut extrap_summed = 0isize;
    let mut pre_extrap_summed = 0isize;
    for l in input.lines() {
        let nums = l
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();

        let mut diffs = vec![nums];
        loop {
            let last = diffs.last().unwrap();
            if last.iter().all(|v| *v == 0) {
                break;
            }
            let mut next_diffs = Vec::<isize>::new();
            for i in 0..last.len() - 1 {
                next_diffs.push(last[i + 1] - last[i]);
            }
            diffs.push(next_diffs);
        }

        diffs.last_mut().unwrap().push(0);

        for i in (0..diffs.len() - 1).rev() {
            let next_last = *diffs[i + 1].last().unwrap();
            let cur = &mut diffs[i];
            cur.push(cur.last().unwrap() + next_last);
        }
        extrap_summed += diffs.first().unwrap().last().unwrap();

        diffs.last_mut().unwrap().insert(0, 0);
        for i in (0..diffs.len() - 1).rev() {
            let next_first = *diffs[i + 1].first().unwrap();
            let cur = &mut diffs[i];
            cur.insert(0, cur.first().unwrap() - next_first);
        }
        pre_extrap_summed += diffs.first().unwrap().first().unwrap();
    }

    (format!("{extrap_summed}"), format!("{pre_extrap_summed}"))
}
