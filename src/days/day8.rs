use std::collections::BTreeMap;

use regex::Regex;

enum Dir {
    L,
    R,
}

pub fn main(input: &str) -> (String, String) {
    let mut lines = input.lines();
    let dirs = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Dir::L,
            'R' => Dir::R,
            other => panic!("{other}"),
        })
        .collect::<Vec<_>>();
    lines.next();

    let mut nodes = BTreeMap::<String, (String, String)>::new();
    let node_re = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    for line in lines {
        let c = node_re.captures(line).unwrap();
        assert!(c.len() == 4);
        let cur = c.get(1).unwrap().as_str().to_owned();
        let left = c.get(2).unwrap().as_str().to_owned();
        let right = c.get(3).unwrap().as_str().to_owned();
        nodes.insert(cur.to_owned(), (left.to_owned(), right.to_owned()));
    }

    fn calc_count<'a, P>(
        nodes: &'a BTreeMap<String, (String, String)>,
        dirs: &Vec<Dir>,
        start_node: &'a str,
        is_end_node: P,
    ) -> (&'a str, usize)
    where
        P: Fn(&str) -> bool,
    {
        let mut cur_node = start_node;

        let mut count = 0usize;
        loop {
            let dir = &dirs[count % dirs.len()];
            count += 1;

            let next_nodes = nodes.get(cur_node);
            let next_nodes = match next_nodes {
                Some(n) => n,
                None => return ("", usize::MAX),
            };
            cur_node = match dir {
                Dir::L => &next_nodes.0,
                Dir::R => &next_nodes.1,
            };
            if is_end_node(cur_node) {
                return (cur_node, count);
            }
        }
    }

    let count = calc_count(&nodes, &dirs, "AAA", |f| f == "ZZZ").1;

    let cur_nodes = nodes
        .iter()
        .filter(|(k, _)| k.ends_with("A"))
        .map(|(k, _)| k.as_str())
        .collect::<Vec<_>>();

    let cur_node_counts = cur_nodes
        .iter()
        .map(|n| calc_count(&nodes, &dirs, n, |f| f.ends_with("Z")).1)
        .collect::<Vec<_>>();

    let mut count2 = 0usize;
    let first = *cur_node_counts.first().unwrap();
    loop {
        count2 += first;
        if cur_node_counts.iter().all(|n| count2 % n == 0) {
            break;
        }
    }

    (format!("{count}"), format!("{count2}"))
}
