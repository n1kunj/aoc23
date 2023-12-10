use std::fmt::Display;

#[derive(Copy, Clone, Debug)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    GR,
    AN,
}

struct Tiles {
    rows: Vec<Vec<Tile>>,
}

impl Display for Tiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for tile in row.iter() {
                let c = match tile {
                    Tile::NS => '║',
                    Tile::EW => '═',
                    Tile::NE => '╚',
                    Tile::NW => '╝',
                    Tile::SW => '╗',
                    Tile::SE => '╔',
                    Tile::GR => ' ',
                    Tile::AN => '?',
                };
                std::fmt::Write::write_char(f, c)?;
            }
            std::fmt::Write::write_char(f, '\n')?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

fn to_dir(t: Tile, from_dir: Dir) -> Option<Dir> {
    match (t, from_dir) {
        (Tile::NS, Dir::N) => Some(Dir::S),
        (Tile::NS, Dir::E) => None,
        (Tile::NS, Dir::S) => Some(Dir::N),
        (Tile::NS, Dir::W) => None,
        (Tile::EW, Dir::N) => None,
        (Tile::EW, Dir::E) => Some(Dir::W),
        (Tile::EW, Dir::S) => None,
        (Tile::EW, Dir::W) => Some(Dir::E),
        (Tile::NE, Dir::N) => Some(Dir::E),
        (Tile::NE, Dir::E) => Some(Dir::N),
        (Tile::NE, Dir::S) => None,
        (Tile::NE, Dir::W) => None,
        (Tile::NW, Dir::N) => Some(Dir::W),
        (Tile::NW, Dir::E) => None,
        (Tile::NW, Dir::S) => None,
        (Tile::NW, Dir::W) => Some(Dir::N),
        (Tile::SW, Dir::N) => None,
        (Tile::SW, Dir::E) => None,
        (Tile::SW, Dir::S) => Some(Dir::W),
        (Tile::SW, Dir::W) => Some(Dir::S),
        (Tile::SE, Dir::N) => None,
        (Tile::SE, Dir::E) => Some(Dir::S),
        (Tile::SE, Dir::S) => Some(Dir::E),
        (Tile::SE, Dir::W) => None,
        (Tile::GR, Dir::N) => None,
        (Tile::GR, Dir::E) => None,
        (Tile::GR, Dir::S) => None,
        (Tile::GR, Dir::W) => None,
        (Tile::AN, Dir::N) => None,
        (Tile::AN, Dir::E) => None,
        (Tile::AN, Dir::S) => None,
        (Tile::AN, Dir::W) => None,
    }
}

pub fn main(input: &str) -> (String, String) {
    let mut rows = Vec::<Vec<Tile>>::new();
    for l in input.lines() {
        let mut row = Vec::<Tile>::new();
        for c in l.chars() {
            let tile = match c {
                '|' => Tile::NS,
                '-' => Tile::EW,
                'L' => Tile::NE,
                'J' => Tile::NW,
                '7' => Tile::SW,
                'F' => Tile::SE,
                '.' => Tile::GR,
                'S' => Tile::AN,
                other => panic!("{other}"),
            };
            row.push(tile);
        }
        rows.push(row);
    }
    let t = Tiles { rows };

    fn get_path(t: &Tiles, start: (usize, usize)) -> (Vec<(usize, usize)>, Tile) {
        assert!(matches!(t.rows[start.1][start.0], Tile::AN));
        for possible_tile in [Tile::NS, Tile::EW, Tile::NE, Tile::NW, Tile::SW, Tile::SE].iter() {
            for possible_dir in [Dir::N, Dir::E] {
                let mut pos = start;
                let mut tile = possible_tile;
                let mut from_dir = possible_dir;
                let mut pos_history = Vec::<(usize, usize)>::new();
                loop {
                    let next_dir = match to_dir(*tile, from_dir) {
                        None => break,
                        Some(d) => d,
                    };
                    let next_pos = match next_dir {
                        Dir::N => (pos.0, pos.1.wrapping_sub(1)),
                        Dir::E => (pos.0.wrapping_add(1), pos.1),
                        Dir::S => (pos.0, pos.1.wrapping_add(1)),
                        Dir::W => (pos.0.wrapping_sub(1), pos.1),
                    };
                    let next_tile = match t.rows.get(next_pos.1) {
                        Some(r) => match r.get(next_pos.0) {
                            Some(tile) => tile,
                            None => break,
                        },
                        None => break,
                    };
                    let next_from_dir = match next_dir {
                        Dir::N => Dir::S,
                        Dir::E => Dir::W,
                        Dir::S => Dir::N,
                        Dir::W => Dir::E,
                    };
                    pos_history.push(next_pos);
                    pos = next_pos;
                    tile = next_tile;
                    from_dir = next_from_dir;

                    if pos == start {
                        assert!(matches!(tile, Tile::AN));
                        if to_dir(*possible_tile, from_dir).is_some() {
                            return (pos_history, *possible_tile);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        panic!()
    }

    let an_start = || -> (usize, usize) {
        for (y, row) in t.rows.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if matches!(tile, Tile::AN) {
                    return (x, y);
                }
            }
        }
        panic!();
    }();

    let (path, an_tile) = get_path(&t, an_start);
    let num_steps = path.len() / 2;

    #[derive(Debug)]
    enum Status {
        IsLoop,
        Untested,
        Inside,
        Outside,
    }

    let mut s = Vec::<Vec<Status>>::new();
    for row in t.rows.iter() {
        let mut v = Vec::<Status>::new();
        v.resize_with(row.len(), || Status::Untested);
        s.push(v);
    }

    for pos in path.iter() {
        s[pos.1][pos.0] = Status::IsLoop;
    }

    let rows = s.len();
    let cols = s[0].len();

    for y in 0..rows {
        for x in 0..cols {
            if !matches!(s[y][x], Status::Untested) {
                continue;
            }
            // Go west until we're OOB or hit an outside tile
            let mut in_score = 0isize;
            let mut next_pos = (x, y);
            loop {
                next_pos = (next_pos.0.wrapping_sub(1), next_pos.1);
                let tile = t.rows[next_pos.1].get(next_pos.0);
                if let Some(mut t) = tile {
                    if matches!(t, Tile::AN) {
                        t = &an_tile;
                    }
                    let st = &s[next_pos.1][next_pos.0];
                    if matches!(st, Status::Outside) {
                        break;
                    }
                    if matches!(st, Status::IsLoop) {
                        in_score += match t {
                            Tile::NS => 2,
                            Tile::EW => 0,
                            Tile::NE => 1,
                            Tile::NW => -1,
                            Tile::SW => 1,
                            Tile::SE => -1,
                            Tile::GR => 0,
                            Tile::AN => panic!(),
                        };
                    }
                } else {
                    break;
                }
            }
            if in_score % 4 == 0 {
                s[y][x] = Status::Outside;
            } else {
                s[y][x] = Status::Inside;
            }
        }
    }

    let mut inside_count = 0usize;
    for (y, row) in t.rows.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if matches!(s[y][x], Status::Inside) {
                inside_count += 1;
            }
        }
    }

    (format!("{num_steps}"), format!("{inside_count}"))
}
