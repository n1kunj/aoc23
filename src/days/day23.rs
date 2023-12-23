use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    SlopeUp,
    SlopeDown,
    SlopeLeft,
    SlopeRight,
}

pub fn main(input: &str) -> (String, String) {
    let mut map = Vec::<Vec<Tile>>::new();
    for l in input.lines() {
        let mut row = Vec::<Tile>::new();
        for c in l.chars() {
            let t = match c {
                '#' => Tile::Forest,
                '.' => Tile::Path,
                '^' => Tile::SlopeUp,
                'v' => Tile::SlopeDown,
                '<' => Tile::SlopeLeft,
                '>' => Tile::SlopeRight,
                other => panic!("{other}"),
            };
            row.push(t);
        }
        map.push(row);
    }
    let map = map;

    let start = || -> (usize, usize) {
        for (i, t) in map[0].iter().enumerate() {
            if matches!(t, Tile::Path) {
                return (i, 0);
            }
        }
        panic!();
    }();

    let end = || -> (usize, usize) {
        for (i, t) in map.last().unwrap().iter().enumerate() {
            if matches!(t, Tile::Path) {
                return (i, map.len() - 1);
            }
        }
        panic!();
    }();

    #[derive(Debug, Clone)]
    struct Head {
        pos: (usize, usize),
        visited: HashMap<(usize, usize), usize>,
    }

    impl Head {
        fn move_to(&mut self, pos: (usize, usize)) {
            assert!(!self.visited.contains_key(&pos));
            self.visited.insert(self.pos, self.visited.len());
            self.pos = pos;
        }

        fn possible_moves(
            &self,
            map: &Vec<Vec<Tile>>,
            ignore_slopes: bool,
        ) -> [Option<(usize, usize)>; 4] {
            let mut ret = [None::<(usize, usize)>; 4];
            let mut idx = 0usize;
            let mut push = |pos: (usize, usize)| {
                ret[idx] = Some(pos);
                idx += 1;
            };

            let at = |pos: (usize, usize)| -> Option<&Tile> { map.get(pos.1)?.get(pos.0) };

            let (x, y) = self.pos;
            let t = at(self.pos).unwrap();
            let t = if ignore_slopes {
                match t {
                    Tile::Path => Tile::Path,
                    Tile::Forest => Tile::Forest,
                    Tile::SlopeUp => Tile::Path,
                    Tile::SlopeDown => Tile::Path,
                    Tile::SlopeLeft => Tile::Path,
                    Tile::SlopeRight => Tile::Path,
                }
            } else {
                *t
            };
            let up = (x, y.wrapping_sub(1));
            let down = (x, y + 1);
            let left = (x.wrapping_sub(1), y);
            let right = (x + 1, y);
            let next_positions = match t {
                Tile::Path => [Some(up), Some(down), Some(left), Some(right)],
                Tile::Forest => panic!(),
                Tile::SlopeUp => [Some(up), None, None, None],
                Tile::SlopeDown => [Some(down), None, None, None],
                Tile::SlopeLeft => [Some(left), None, None, None],
                Tile::SlopeRight => [Some(right), None, None, None],
            };
            for npos in next_positions {
                if let Some(npos) = npos {
                    if self.visited.contains_key(&npos) {
                        continue;
                    }
                    if let Some(t) = at(npos) {
                        if !matches!(t, Tile::Forest) {
                            push(npos);
                        }
                    }
                }
            }

            ret
        }
    }

    // Convert the map into a graph
    #[derive(Debug)]
    struct Node {
        connects_to: Vec<(usize, usize)>,
    }

    let calc_graph = |ignore_slopes: bool| -> Vec<Node> {
        let mut pending_nodes = vec![start, end];
        let mut next_pending_nodes = Vec::<(usize, usize)>::new();

        let mut processed_nodes = HashMap::<(usize, usize), usize>::new();

        struct TempNode {
            connects_to: Vec<((usize, usize), usize)>,
        }

        let mut temp_nodes = Vec::<TempNode>::new();

        while !pending_nodes.is_empty() {
            for pn in pending_nodes.drain(..) {
                if processed_nodes.contains_key(&pn) {
                    continue;
                }
                // Walk until we find the next node that has more than one option
                let mut heads = Vec::<Head>::new();
                let mut pmoves = Vec::<(usize, usize)>::new();
                let mut connects_to = Vec::<((usize, usize), usize)>::new();

                heads.push(Head {
                    pos: pn,
                    visited: HashMap::new(),
                });
                let mut next_heads = Vec::<Head>::new();
                while !heads.is_empty() {
                    for head in heads.drain(..) {
                        let possible_moves = head.possible_moves(&map, ignore_slopes);
                        pmoves.clear();
                        for pmove in possible_moves {
                            if let Some(pmove) = pmove {
                                pmoves.push(pmove);
                            }
                        }
                        if pmoves.len() > 1 && head.pos != pn
                            || pmoves.len() == 0 && (head.pos == start || head.pos == end)
                        {
                            connects_to.push((head.pos, head.visited.len()));
                        } else {
                            for pmove in pmoves.iter() {
                                let mut h = head.clone();
                                h.move_to(*pmove);
                                next_heads.push(h);
                            }
                        }
                    }
                    heads.extend(next_heads.drain(..));
                }
                next_pending_nodes.extend(connects_to.iter().map(|(k, _)| k));
                processed_nodes.insert(pn, processed_nodes.len());
                temp_nodes.push(TempNode { connects_to });
            }
            pending_nodes.extend(next_pending_nodes.drain(..));
        }

        let mut nodes = Vec::<Node>::new();
        for temp_node in temp_nodes {
            let connects_to = temp_node
                .connects_to
                .iter()
                .map(|(pos, dist)| (processed_nodes[pos], *dist))
                .collect::<Vec<_>>();
            nodes.push(Node { connects_to })
        }
        nodes
    };

    let start_idx = 0;
    let end_idx = 1;

    let first_graph = calc_graph(false);
    let second_graph = calc_graph(true);

    let calc_max_path = |graph: &Vec<Node>| -> usize {
        assert!(graph.len() < u64::BITS as usize);
        #[derive(Debug, Clone)]
        struct Path {
            idx: usize,
            visited: usize,
            distance: usize,
        }

        impl Path {
            fn move_to(&mut self, dst: usize, dist: usize) {
                assert!(!self.visited(dst));
                self.visited |= 1 << dst;
                self.idx = dst;
                self.distance += dist;
            }

            fn visited(&self, dst: usize) -> bool {
                (self.visited & 1 << dst) != 0
            }
        }

        let mut paths = Vec::<Path>::new();
        paths.push(Path {
            idx: start_idx,
            visited: 0,
            distance: 0,
        });
        let mut next_paths = Vec::<Path>::new();
        let mut max_completed = None::<Path>;
        let mut num_completed = 0usize;
        let mut num_terminated = 0usize;
        while !paths.is_empty() {
            let mut process_path = |path: Path| {
                let node = &graph[path.idx];
                for (dst, dist) in node.connects_to.iter() {
                    if !path.visited(*dst) {
                        let mut npath = path.clone();
                        npath.move_to(*dst, *dist);
                        if *dst == end_idx {
                            num_completed += 1;
                            max_completed = match max_completed.take() {
                                Some(v) => {
                                    if npath.distance > v.distance {
                                        Some(npath)
                                    } else {
                                        Some(v)
                                    }
                                }
                                None => Some(npath),
                            };
                        } else {
                            next_paths.push(npath);
                        }
                    } else {
                        num_terminated += 1;
                    }
                }
            };
            process_path(paths.pop().unwrap());
            paths.extend(next_paths.drain(..));
        }

        max_completed.unwrap().distance
    };

    let max_first = calc_max_path(&first_graph);
    let max_second = calc_max_path(&second_graph);

    (format!("{max_first}"), format!("{max_second}"))
}
