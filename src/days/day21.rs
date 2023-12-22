use std::collections::HashSet;

enum Tile {
    Start,
    Plot,
    Rock,
}

pub fn main(input: &str) -> (String, String) {
    let mut rows = Vec::<Vec<Tile>>::new();
    for l in input.lines() {
        let mut row = Vec::<Tile>::new();
        for c in l.chars() {
            let tile = match c {
                '.' => Tile::Plot,
                '#' => Tile::Rock,
                'S' => Tile::Start,
                other => panic!("{other}"),
            };
            row.push(tile);
        }
        rows.push(row);
    }
    let rows = rows;

    let start = || -> (usize, usize) {
        for (y, row) in rows.iter().enumerate() {
            for (x, t) in row.iter().enumerate() {
                if matches!(t, Tile::Start) {
                    return (x, y);
                }
            }
        }
        panic!();
    }();

    let get_tile_at = |x: usize, y: usize| -> Option<&Tile> { rows.get(y)?.get(x) };

    let calc_reachable_after_its = |start: (usize, usize)| -> Vec<usize> {
        let mut cur_positions = HashSet::<(usize, usize)>::new();
        let mut next_positions = HashSet::<(usize, usize)>::new();

        cur_positions.insert(start);

        let mut reachable_after_its = Vec::<usize>::new();
        reachable_after_its.push(cur_positions.len());

        loop {
            for (x, y) in cur_positions.drain() {
                for (nx, ny) in [
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                ] {
                    if let Some(nt) = get_tile_at(nx, ny) {
                        if matches!(nt, Tile::Rock) {
                            continue;
                        }
                        next_positions.insert((nx, ny));
                    }
                }
            }
            cur_positions.extend(next_positions.drain());
            reachable_after_its.push(cur_positions.len());
            if reachable_after_its.len() > 3
                && reachable_after_its[reachable_after_its.len() - 1]
                    == reachable_after_its[reachable_after_its.len() - 3]
            {
                break;
            }
        }
        reachable_after_its
    };

    let from_start = calc_reachable_after_its(start);

    let reachable_plots = get_reachable_after_steps(64, &from_start);

    let from_tl = calc_reachable_after_its((0, 0));
    let from_t = calc_reachable_after_its((start.0, 0));
    let from_tr = calc_reachable_after_its((rows[0].len() - 1, 0));
    let from_r = calc_reachable_after_its((rows[0].len() - 1, start.1));
    let from_br = calc_reachable_after_its((rows[0].len() - 1, rows.len() - 1));
    let from_b = calc_reachable_after_its((start.0, rows.len() - 1));
    let from_bl = calc_reachable_after_its((0, rows.len() - 1));
    let from_l = calc_reachable_after_its((0, start.1));

    fn get_reachable_after_steps(n: usize, precalc: &Vec<usize>) -> usize {
        if let Some(v) = precalc.get(n) {
            *v
        } else {
            let d = n - precalc.len();
            if d % 2 == 1 {
                precalc[precalc.len() - 1]
            } else {
                precalc[precalc.len() - 2]
            }
        }
    }

    let calc_after_steps = |n: usize| -> usize {
        let centre_only = get_reachable_after_steps(n, &from_start);

        let mut total = centre_only;

        let mut it = 0usize;
        loop {
            let steps_to_edges = (rows[0].len() - 1) / 2 + it * rows[0].len();
            if n > steps_to_edges {
                let steps_into_edges = n - steps_to_edges - 1;
                let left = get_reachable_after_steps(steps_into_edges, &from_l);
                let right = get_reachable_after_steps(steps_into_edges, &from_r);
                let up = get_reachable_after_steps(steps_into_edges, &from_t);
                let down = get_reachable_after_steps(steps_into_edges, &from_b);

                total += left + right + up + down;
                it += 1;
            } else {
                break;
            }
        }

        let mut it = 0usize;
        loop {
            let steps_to_corners = rows[0].len() + it * rows[0].len();
            if n > steps_to_corners {
                let steps_into_corners = n - steps_to_corners - 1;
                let bl = get_reachable_after_steps(steps_into_corners, &from_bl);
                let br = get_reachable_after_steps(steps_into_corners, &from_br);
                let tl = get_reachable_after_steps(steps_into_corners, &from_tl);
                let tr = get_reachable_after_steps(steps_into_corners, &from_tr);

                total += (bl + br + tl + tr) * (it + 1);
                it += 1;
            } else {
                break;
            }
        }

        total
    };

    let reachable_plots2 = calc_after_steps(26501365);

    (format!("{reachable_plots}"), format!("{reachable_plots2}"))
}
