use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Plan {
    d: Dir,
    l: usize,
}

#[derive(Debug, Copy, Clone)]
enum Terrain {
    Unknown,
    Edge,
    Interior,
    Exterior,
}

struct Ground {
    rows: Vec<Vec<Terrain>>,
}

pub fn main(input: &str) -> (String, String) {
    let re = Regex::new(r"(.) (\d+) \(#(.....)(.)\)").unwrap();

    let mut plans = Vec::<Plan>::new();
    let mut plans2 = Vec::<Plan>::new();
    for l in input.lines() {
        let captures = re.captures(l).unwrap();
        let d = match captures.get(1).unwrap().as_str() {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            other => panic!("{other}"),
        };
        let l = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        let l2 = usize::from_str_radix(captures.get(3).unwrap().as_str(), 16).unwrap();
        let d2 = match captures.get(4).unwrap().as_str() {
            "3" => Dir::U,
            "1" => Dir::D,
            "2" => Dir::L,
            "0" => Dir::R,
            other => panic!("{other}"),
        };

        plans.push(Plan { d, l });
        plans2.push(Plan { d: d2, l: l2 });
    }

    let mut g = Ground {
        rows: vec![vec![Terrain::Unknown]],
    };

    let mut pos = (0usize, 0usize);
    for p in plans.iter() {
        let l = p.l;
        for _ in 0..l {
            pos = match p.d {
                Dir::U => (pos.0, pos.1.wrapping_sub(1)),
                Dir::D => (pos.0, pos.1 + 1),
                Dir::L => (pos.0.wrapping_sub(1), pos.1),
                Dir::R => (pos.0 + 1, pos.1),
            };
            if pos.0 == usize::MAX {
                for row in g.rows.iter_mut() {
                    row.insert(0, Terrain::Unknown);
                }
                pos.0 = 0;
            }
            while pos.0 >= g.rows[0].len() {
                for row in g.rows.iter_mut() {
                    row.push(Terrain::Unknown);
                }
            }
            if pos.1 == usize::MAX {
                g.rows.insert(0, vec![Terrain::Unknown; g.rows[0].len()]);
                pos.1 = 0;
            }
            while pos.1 >= g.rows.len() {
                g.rows.push(vec![Terrain::Unknown; g.rows[0].len()]);
            }

            g.rows[pos.1][pos.0] = Terrain::Edge;
        }
    }

    let mut any_changes = true;
    while any_changes {
        any_changes = false;
        for y in 0..g.rows.len() {
            for x in 0..g.rows[0].len() {
                if !matches!(g.rows[y][x], Terrain::Unknown) {
                    continue;
                }

                for (nx, ny) in [
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                ] {
                    let ot = g.rows.get(ny).and_then(|r| r.get(nx));
                    if ot.is_none() || ot.is_some_and(|t| matches!(t, Terrain::Exterior)) {
                        g.rows[y][x] = Terrain::Exterior;
                        any_changes = true;
                    }
                }
            }
        }
    }

    for row in g.rows.iter_mut() {
        for t in row.iter_mut() {
            if matches!(t, Terrain::Unknown) {
                *t = Terrain::Interior;
            }
        }
    }

    for row in g.rows.iter() {
        for t in row.iter() {
            let c = match t {
                Terrain::Unknown => '?',
                Terrain::Edge => '#',
                Terrain::Interior => 'X',
                Terrain::Exterior => '.',
            };
            print!("{c}");
        }
        println!();
    }

    let mut part_1_count = 0usize;
    for row in g.rows.iter() {
        for t in row.iter() {
            match t {
                Terrain::Unknown => panic!(),
                Terrain::Edge => part_1_count += 1,
                Terrain::Interior => part_1_count += 1,
                Terrain::Exterior => (),
            };
        }
    }

    // Calculate the extents of the plans
    let mut min_x = 0isize;
    let mut max_x = 0isize;
    let mut min_y = 0isize;
    let mut max_y = 0isize;

    let mut hor_lines = Vec::<(isize, isize, isize)>::new();
    let mut ver_lines = Vec::<(isize, isize, isize)>::new();
    let mut pos = (0isize, 0isize);
    for p in plans2.iter() {
        let l = p.l as isize;
        let prev_pos = pos;
        pos = match p.d {
            Dir::U => (pos.0, pos.1 - l),
            Dir::D => (pos.0, pos.1 + l),
            Dir::L => (pos.0 - l, pos.1),
            Dir::R => (pos.0 + l, pos.1),
        };
        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);

        if prev_pos.0 == pos.0 {
            let mut ys = [prev_pos.1, pos.1];
            ys.sort();
            ver_lines.push((pos.0, ys[0], ys[1]))
        } else {
            assert!(prev_pos.1 == pos.1);
            let mut xs = [prev_pos.0, pos.0];
            xs.sort();
            hor_lines.push((pos.1, xs[0], xs[1]));
        }
    }

    hor_lines.sort();
    ver_lines.sort();

    let mut hor_set = HashSet::<(isize, isize, isize)>::new();
    for hl in hor_lines.iter() {
        hor_set.insert(*hl);
    }

    let mut row_counts = Vec::<usize>::new();
    for y in min_y..=max_y {
        let mut count = 0usize;

        let mut next_pair_is_inside = false;
        let mut prev_line: Option<(isize, isize, isize)> = None;
        for cur in ver_lines.iter().filter(|v| v.1 <= y && v.2 >= y) {
            if prev_line.is_none() {
                next_pair_is_inside = true;
                prev_line = Some(*cur);
                count += 1;
                continue;
            }

            let prev = prev_line.unwrap();

            let has_connecting_hor_line = hor_set.contains(&(y, prev.0, cur.0));

            if next_pair_is_inside || has_connecting_hor_line {
                count += (cur.0 - prev.0) as usize;
            } else {
                count += 1;
            }

            if !has_connecting_hor_line {
                next_pair_is_inside = !next_pair_is_inside;
            } else {
                let turns_back = cur.1 == prev.1 || cur.2 == prev.2;
                if turns_back {
                    if next_pair_is_inside {
                        next_pair_is_inside = false;
                    } else {
                        next_pair_is_inside = true;
                    }
                } else {
                    if next_pair_is_inside {
                        next_pair_is_inside = true;
                    } else {
                        next_pair_is_inside = false;
                    }
                }
            }
            prev_line = Some(*cur);
        }
        assert!(!next_pair_is_inside);
        row_counts.push(count);
    }

    let part_2_count = row_counts.iter().sum::<usize>();

    (format!("{part_1_count}"), format!("{part_2_count}"))
}
