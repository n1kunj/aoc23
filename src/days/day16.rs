use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterHor,
    SplitterVer,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn progress(t: Tile, d: Dir) -> [Option<Dir>; 2] {
    match (t, d) {
        (Tile::Empty, Dir::Up) => [Some(Dir::Up), None],
        (Tile::Empty, Dir::Down) => [Some(Dir::Down), None],
        (Tile::Empty, Dir::Left) => [Some(Dir::Left), None],
        (Tile::Empty, Dir::Right) => [Some(Dir::Right), None],
        (Tile::MirrorForward, Dir::Up) => [Some(Dir::Right), None],
        (Tile::MirrorForward, Dir::Down) => [Some(Dir::Left), None],
        (Tile::MirrorForward, Dir::Left) => [Some(Dir::Down), None],
        (Tile::MirrorForward, Dir::Right) => [Some(Dir::Up), None],
        (Tile::MirrorBackward, Dir::Up) => [Some(Dir::Left), None],
        (Tile::MirrorBackward, Dir::Down) => [Some(Dir::Right), None],
        (Tile::MirrorBackward, Dir::Left) => [Some(Dir::Up), None],
        (Tile::MirrorBackward, Dir::Right) => [Some(Dir::Down), None],
        (Tile::SplitterHor, Dir::Up) => [Some(Dir::Left), Some(Dir::Right)],
        (Tile::SplitterHor, Dir::Down) => [Some(Dir::Left), Some(Dir::Right)],
        (Tile::SplitterHor, Dir::Left) => [Some(Dir::Left), None],
        (Tile::SplitterHor, Dir::Right) => [Some(Dir::Right), None],
        (Tile::SplitterVer, Dir::Up) => [Some(Dir::Up), None],
        (Tile::SplitterVer, Dir::Down) => [Some(Dir::Down), None],
        (Tile::SplitterVer, Dir::Left) => [Some(Dir::Up), Some(Dir::Down)],
        (Tile::SplitterVer, Dir::Right) => [Some(Dir::Up), Some(Dir::Down)],
    }
}

fn next_pos(x: usize, y: usize, d: Dir) -> (usize, usize) {
    match d {
        Dir::Up => (x, y.wrapping_sub(1)),
        Dir::Down => (x, y.wrapping_add(1)),
        Dir::Left => (x.wrapping_sub(1), y),
        Dir::Right => (x.wrapping_add(1), y),
    }
}

#[derive(Debug, Clone)]
struct Grid {
    rows: Vec<Vec<Tile>>,
}

fn calc_rays_set(g: &Grid, x: usize, y: usize, d: Dir) -> HashSet<(usize, usize, Dir)> {
    let mut rays = HashSet::<(usize, usize, Dir)>::new();

    let mut to_be_handled = Vec::<(usize, usize, Dir)>::new();
    to_be_handled.push((x, y, d));

    let mut next_to_be_handled = Vec::<(usize, usize, Dir)>::new();

    while to_be_handled.len() > 0 {
        for (x, y, d) in to_be_handled.drain(..) {
            let row = g.rows.get(y);
            if let Some(row) = row {
                let t = row.get(x);
                if let Some(t) = t {
                    if rays.insert((x, y, d)) {
                        let next_dirs = progress(*t, d);
                        for nd in next_dirs.iter() {
                            if let Some(nd) = nd {
                                let (nx, ny) = next_pos(x, y, *nd);
                                next_to_be_handled.push((nx, ny, *nd));
                            }
                        }
                    }
                }
            }
        }
        to_be_handled.extend(next_to_be_handled.drain(..));
    }

    rays
}

fn powered_count(g: &Grid, rays: &HashSet<(usize, usize, Dir)>) -> usize {
    let mut powered_count = 0usize;
    for (y, row) in g.rows.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            for d in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                if rays.contains(&(x, y, d)) {
                    powered_count += 1;
                    break;
                }
            }
        }
    }
    powered_count
}

pub fn main(input: &str) -> (String, String) {
    let mut rows = Vec::<Vec<Tile>>::new();
    for l in input.lines() {
        let row = l
            .chars()
            .map(|c| match c {
                '.' => Tile::Empty,
                '/' => Tile::MirrorForward,
                '\\' => Tile::MirrorBackward,
                '-' => Tile::SplitterHor,
                '|' => Tile::SplitterVer,
                other => panic!("{other}"),
            })
            .collect::<Vec<_>>();
        rows.push(row);
    }

    let g = Grid { rows };

    let first_rays = calc_rays_set(&g, 0, 0, Dir::Right);
    let first_powered_count = powered_count(&g, &first_rays);

    let mut max_powered_count = 0usize;

    let mut update_max_powered_count = |x: usize, y: usize, d: Dir| {
        let rays = calc_rays_set(&g, x, y, d);
        let powered_count = powered_count(&g, &rays);

        if powered_count > max_powered_count {
            max_powered_count = powered_count;
        }
    };

    for x in 0..g.rows[0].len() {
        update_max_powered_count(x, 0, Dir::Down);
        update_max_powered_count(x, g.rows.len() - 1, Dir::Up);
    }
    for y in 0..g.rows.len() {
        update_max_powered_count(0, y, Dir::Right);
        update_max_powered_count(g.rows[0].len() - 1, y, Dir::Left);
    }

    (
        format!("{first_powered_count}"),
        format!("{max_powered_count}"),
    )
}
