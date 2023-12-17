use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone)]
struct Row {
    row: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Map {
    rows: Vec<Row>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn next_pos(x: usize, y: usize, d: Dir) -> (usize, usize) {
    match d {
        Dir::Up => (x, y.wrapping_sub(1)),
        Dir::Down => (x, y.wrapping_add(1)),
        Dir::Left => (x.wrapping_sub(1), y),
        Dir::Right => (x.wrapping_add(1), y),
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct PathNode {
    x: usize,
    y: usize,
    d: Dir,
}

impl PathNode {
    fn new(x: usize, y: usize, d: Dir) -> PathNode {
        PathNode { x, y, d }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Path {
    nodes: Vec<PathNode>,
    cost: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Head {
    x: usize,
    y: usize,
    d: Dir,
    n: usize,
}

impl Head {
    fn new(x: usize, y: usize, d: Dir, n: usize) -> Head {
        Head { x, y, d, n }
    }
}

fn possible_dirs(d: Dir, moves_in_dir: usize) -> [Option<Dir>; 4] {
    if moves_in_dir == 0 {
        return [
            Some(Dir::Up),
            Some(Dir::Down),
            Some(Dir::Left),
            Some(Dir::Right),
        ];
    }
    let too_many_moves = moves_in_dir >= 3;
    match (d, too_many_moves) {
        (Dir::Up, true) => [Some(Dir::Left), Some(Dir::Right), None, None],
        (Dir::Up, false) => [Some(Dir::Left), Some(Dir::Right), Some(Dir::Up), None],
        (Dir::Down, true) => [Some(Dir::Left), Some(Dir::Right), None, None],
        (Dir::Down, false) => [Some(Dir::Left), Some(Dir::Right), Some(Dir::Down), None],
        (Dir::Left, true) => [Some(Dir::Up), Some(Dir::Down), None, None],
        (Dir::Left, false) => [Some(Dir::Up), Some(Dir::Down), Some(Dir::Left), None],
        (Dir::Right, true) => [Some(Dir::Up), Some(Dir::Down), None, None],
        (Dir::Right, false) => [Some(Dir::Up), Some(Dir::Down), Some(Dir::Right), None],
    }
}

fn ultra_dirs(d: Dir, moves_in_dir: usize) -> [Option<Dir>; 4] {
    if moves_in_dir == 0 {
        [
            Some(Dir::Up),
            Some(Dir::Down),
            Some(Dir::Left),
            Some(Dir::Right),
        ]
    } else if moves_in_dir < 4 {
        [Some(d), None, None, None]
    } else if moves_in_dir < 10 {
        match d {
            Dir::Up => [Some(Dir::Left), Some(Dir::Right), Some(Dir::Up), None],
            Dir::Down => [Some(Dir::Left), Some(Dir::Right), Some(Dir::Down), None],
            Dir::Left => [Some(Dir::Up), Some(Dir::Down), Some(Dir::Left), None],
            Dir::Right => [Some(Dir::Up), Some(Dir::Down), Some(Dir::Right), None],
        }
    } else {
        match d {
            Dir::Up => [Some(Dir::Left), Some(Dir::Right), None, None],
            Dir::Down => [Some(Dir::Left), Some(Dir::Right), None, None],
            Dir::Left => [Some(Dir::Up), Some(Dir::Down), None, None],
            Dir::Right => [Some(Dir::Up), Some(Dir::Down), None, None],
        }
    }
}

pub fn main(input: &str) -> (String, String) {
    let mut rows = Vec::<Row>::new();
    for l in input.lines() {
        rows.push(Row {
            row: l
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>(),
        });
    }

    let m = Map { rows };

    fn calc_best_path<T>(m: &Map, start: (usize, usize), end: (usize, usize), dirs_fn: T) -> Path
    where
        T: Fn(Dir, usize) -> [Option<Dir>; 4],
    {
        fn heuristic(pos: (usize, usize), end: (usize, usize)) -> usize {
            end.0.abs_diff(pos.0) + end.1.abs_diff(pos.1)
        }

        #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
        struct OpenSetHead {
            f: usize,
            h: Head,
        }

        let cost_of =
            |pos: Head| -> Option<usize> { m.rows.get(pos.y)?.row.get(pos.x).map(|v| *v) };

        let mut came_from = HashMap::<Head, (Head, Dir)>::new();

        let mut g_score = HashMap::<Head, usize>::new();

        let heuristic = |pos: (usize, usize)| -> usize { heuristic(pos, end) };

        let mut f_score = HashMap::<Head, usize>::new();

        let mut open_set = BTreeSet::<OpenSetHead>::new();

        let start_head = Head::new(start.0, start.1, Dir::Down, 0);
        g_score.insert(start_head, 0);
        f_score.insert(start_head, heuristic(start));
        open_set.insert(OpenSetHead {
            f: heuristic(start),
            h: start_head,
        });

        while !open_set.is_empty() {
            let OpenSetHead { f: _, h: current } = open_set.pop_first().unwrap();
            let pos = (current.x, current.y);

            if end == pos {
                let mut nodes = Vec::<PathNode>::new();
                let mut cost = 0usize;
                let mut cur_pos = current;
                loop {
                    match came_from.get(&cur_pos) {
                        Some((from, pd)) => {
                            cost += cost_of(cur_pos).unwrap();
                            nodes.push(PathNode::new(cur_pos.x, cur_pos.y, *pd));
                            cur_pos = *from;
                        }
                        None => {
                            return Path { cost, nodes };
                        }
                    }
                }
            }

            let g_score_cur = *g_score.get(&current).unwrap();
            for pd in dirs_fn(current.d, current.n) {
                if let Some(nd) = pd {
                    let npos = next_pos(current.x, current.y, nd);
                    let nn = if current.d == nd { current.n + 1 } else { 1 };
                    let next_current = Head::new(npos.0, npos.1, nd, nn);
                    if let Some(cost) = cost_of(next_current) {
                        let tentative_g_score = g_score_cur + cost;
                        let neighbor_g_score = *g_score.get(&next_current).unwrap_or(&usize::MAX);
                        if tentative_g_score < neighbor_g_score {
                            came_from.insert(next_current, (current, nd));
                            g_score.insert(next_current, tentative_g_score);
                            let h = heuristic(npos);
                            let f = h + tentative_g_score;
                            let old_f = f_score.insert(next_current, f);
                            if let Some(old_f) = old_f {
                                open_set.remove(&OpenSetHead {
                                    f: old_f,
                                    h: next_current,
                                });
                            }
                            open_set.insert(OpenSetHead { f, h: next_current });
                        }
                    }
                }
            }
        }

        panic!();
    }

    let print_path = |p: &Path| {
        let mut hm = HashMap::<(usize, usize), Dir>::new();
        for n in p.nodes.iter() {
            hm.insert((n.x, n.y), n.d);
        }
        let mut cp = 0usize;

        for y in 0..m.rows.len() {
            for x in 0..m.rows[0].row.len() {
                let tc = m.rows[y].row[x];
                if let Some(d) = hm.get(&(x, y)) {
                    print!(
                        "{}",
                        match d {
                            Dir::Up => '^',
                            Dir::Down => 'v',
                            Dir::Left => '<',
                            Dir::Right => '>',
                        }
                    );
                    cp += tc;
                } else {
                    print!("{}", tc);
                }
            }
            println!();
        }
        println!("{}", p.cost);
        println!();
        assert!(p.cost == cp);
    };

    let end = (m.rows[0].row.len() - 1, m.rows.len() - 1);

    let first_best_path = calc_best_path(&m, (0, 0), end, possible_dirs);
    let first_min_cost = first_best_path.cost;
    print_path(&first_best_path);

    let second_best_path = calc_best_path(&m, (0, 0), end, ultra_dirs);
    let second_min_cost = second_best_path.cost;
    print_path(&second_best_path);

    (format!("{first_min_cost}"), format!("{second_min_cost}"))
}
