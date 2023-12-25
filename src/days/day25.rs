use std::{
    cmp::Reverse,
    collections::{BTreeSet, HashMap},
    iter::once,
};

use combinatorial::Combinations;

pub fn main(input: &str) -> (String, String) {
    let mut hm = HashMap::<String, (usize, BTreeSet<String>)>::new();
    for l in input.lines() {
        let mut p = l.split(":");
        let first = p.next().unwrap();
        let second = p.next().unwrap().split_whitespace();
        for sec in second {
            let mut insert = |f: &str, s: &str| {
                let entries = hm.len();
                hm.entry(f.to_owned())
                    .or_insert((entries, BTreeSet::new()))
                    .1
                    .insert(s.to_owned());
            };
            insert(first, sec);
            insert(sec, first);
        }
    }
    let hm = hm;

    let mut idx_to_name = vec![""; hm.len()];
    for (n, (i, _)) in hm.iter() {
        idx_to_name[*i] = n.as_str();
    }

    let mut idx_to_conns = vec![Vec::<usize>::new(); hm.len()];
    for (idx, name) in idx_to_name.iter().enumerate() {
        let (_, cs) = &hm[*name];
        for c in cs.iter() {
            idx_to_conns[idx].push(hm[c].0);
        }
    }

    let idx_to_name = idx_to_name;
    let idx_to_conns = idx_to_conns;

    let calc_shortest_paths_to =
        |start: usize, ignored_paths: &[(usize, usize)]| -> HashMap<usize, Vec<usize>> {
            struct Head {
                pos: usize,
                path: Vec<usize>,
            }

            let mut heads = vec![Head {
                pos: start,
                path: Vec::new(),
            }];

            let mut next_heads = Vec::<Head>::new();

            let mut shortest_path_to = HashMap::<usize, Vec<usize>>::new();

            while !heads.is_empty() {
                let mut handle_head = |h: Head| {
                    match shortest_path_to.get(&h.pos) {
                        Some(path) => {
                            if path.len() < h.path.len() {
                                return;
                            }
                        }
                        None => (),
                    }
                    shortest_path_to.insert(h.pos, h.path.clone());
                    for c in idx_to_conns[h.pos].iter() {
                        let mut conn = [h.pos, *c];
                        conn.sort();
                        if ignored_paths.contains(&(conn[0], conn[1])) {
                            continue;
                        }

                        let npos: usize = *c;
                        let mut npath = h.path.clone();
                        npath.push(h.pos);
                        next_heads.push(Head {
                            pos: npos,
                            path: npath,
                        });
                    }
                };
                for head in heads.drain(..) {
                    handle_head(head);
                }
                heads.extend(next_heads.drain(..));
            }
            shortest_path_to
        };

    fn conn_appearances_to_vec(
        conn_appearances: &HashMap<(usize, usize), usize>,
    ) -> Vec<((usize, usize), usize)> {
        let mut conn_appearances = conn_appearances
            .iter()
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>();
        conn_appearances.sort_by_key(|n| Reverse(n.1));
        conn_appearances
    }

    fn calc_conn_appearances(
        shortest_paths_to: &HashMap<usize, Vec<usize>>,
    ) -> HashMap<(usize, usize), usize> {
        let mut conn_appearances = HashMap::<(usize, usize), usize>::new();
        for (dst, path) in shortest_paths_to.iter() {
            let mut prev = None::<usize>;
            for p in path.iter().chain(once(dst)) {
                if let Some(prev) = prev {
                    let mut conn = [prev, *p];
                    conn.sort();
                    let conn = (conn[0], conn[1]);
                    conn_appearances
                        .entry(conn)
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
                prev = Some(*p);
            }
        }
        conn_appearances
    }

    let mut ignored_paths = Vec::<(usize, usize)>::new();
    let mut idx = 0usize;
    loop {
        let mut agg_appearances = HashMap::<(usize, usize), usize>::new();
        for _ in 0..10 {
            let shortest_path_to = calc_shortest_paths_to(idx % idx_to_name.len(), &ignored_paths);
            idx += 1;

            if shortest_path_to.len() != idx_to_name.len() {
                assert!(ignored_paths.len() >= 3);
                break;
            }
            let conn_appearances = calc_conn_appearances(&shortest_path_to);
            let agg_conn_appearances = conn_appearances_to_vec(&conn_appearances);
            for (conn, count) in agg_conn_appearances.iter() {
                agg_appearances
                    .entry(*conn)
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            }
        }
        if agg_appearances.len() == 0 {
            break;
        }

        let agg_conn_appearances = conn_appearances_to_vec(&agg_appearances);
        let most_common = agg_conn_appearances[0];
        ignored_paths.push(most_common.0);
    }

    // Pick all combinations of three of the ignored paths - only one combination will reduce the reachable set
    let wires = || -> Vec<(usize, usize)> {
        for combo in Combinations::of_size(ignored_paths, 3) {
            let shortest_path_to = calc_shortest_paths_to(0, &combo);
            if shortest_path_to.len() != idx_to_name.len() {
                return combo;
            }
        }
        panic!();
    }();

    let shortest_path_to = calc_shortest_paths_to(0, &wires);
    let reachable = shortest_path_to.len();
    let unreachable = idx_to_name.len() - reachable;

    let product = reachable * unreachable;

    (format!("{product}"), format!(""))
}
