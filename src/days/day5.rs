use std::iter::once;

use regex::Regex;

#[derive(Clone, Debug)]
struct CatRange {
    dst_start: usize,
    src_start: usize,
    len: usize,
}

#[derive(Debug)]
struct Map {
    _src: String,
    _dst: String,
    ranges: Vec<CatRange>,
}

impl Map {
    fn new(src: String, dst: String, mut ranges: Vec<CatRange>) -> Map {
        ranges.sort_by(|a, b| a.src_start.cmp(&b.src_start));
        let mut new_ranges = Vec::<CatRange>::new();
        let mut cur_src = 0usize;
        for r in ranges.iter() {
            assert!(cur_src <= r.src_start);
            if cur_src < r.src_start {
                new_ranges.push(CatRange {
                    src_start: cur_src,
                    dst_start: cur_src,
                    len: r.src_start - cur_src,
                });
            }
            new_ranges.push(r.clone());
            cur_src = r.src_start + r.len;
        }
        new_ranges.push(CatRange {
            src_start: cur_src,
            dst_start: cur_src,
            len: usize::MAX - cur_src,
        });

        Map {
            _src: src.to_owned(),
            _dst: dst.to_owned(),
            ranges: new_ranges,
        }
    }
}

pub fn main(input: &str) -> (String, String) {
    let seeds_re = Regex::new(r"seeds: (.+)").unwrap();
    let mut lines = input.lines();

    let seeds_cap = seeds_re.captures(lines.next().unwrap()).unwrap();
    assert!(seeds_cap.len() == 2);
    let seeds = seeds_cap
        .get(1)
        .unwrap()
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = Vec::<Map>::new();
    let map_re = Regex::new(r"(.+)-to-(.+) map:").unwrap();
    let mut cur_map: Option<(String, String)> = None;
    let mut cur_ranges = Vec::<CatRange>::new();
    for line in lines.chain(once("")) {
        if line.is_empty() {
            if let Some((src, dst)) = &cur_map {
                maps.push(Map::new(src.to_owned(), dst.to_owned(), cur_ranges.clone()));
            }
            cur_map = None;
            cur_ranges.clear();
        } else if cur_map.is_none() {
            for m in map_re.captures_iter(line) {
                cur_map = Some((
                    m.get(1).unwrap().as_str().to_owned(),
                    m.get(2).unwrap().as_str().to_owned(),
                ));
            }
        } else {
            let mut elems = line.split_whitespace();
            cur_ranges.push(CatRange {
                dst_start: elems.next().unwrap().parse::<usize>().unwrap(),
                src_start: elems.next().unwrap().parse::<usize>().unwrap(),
                len: elems.next().unwrap().parse::<usize>().unwrap(),
            });
            assert!(elems.next().is_none());
        }
    }
    assert!(cur_map.is_none());
    assert!(cur_ranges.len() == 0);

    let mut min_single_loc = usize::MAX;
    for s in seeds.iter() {
        let mut cur_loc = *s;
        for m in maps.iter() {
            for r in m.ranges.iter() {
                if cur_loc < r.src_start {
                    continue;
                }
                if cur_loc > r.src_start + r.len {
                    continue;
                }
                cur_loc = cur_loc - r.src_start + r.dst_start;
                break;
            }
        }
        min_single_loc = min_single_loc.min(cur_loc);
    }

    assert!(seeds.len() % 2 == 0);
    let seed_ranges = (0..seeds.len())
        .step_by(2)
        .map(|i| (seeds[i], seeds[i + 1]))
        .collect::<Vec<_>>();

    let mut cur_ranges = seed_ranges.clone();

    for m in maps.iter() {
        let mut next_ranges = Vec::<(usize, usize)>::new();
        for range in cur_ranges.iter() {
            let mut start = range.0;
            let mut len = range.1;

            for r in m.ranges.iter() {
                if start >= r.src_start && start < r.src_start + r.len {
                    let off = start - r.src_start;

                    let new_start = usize::min(r.src_start + r.len, start + len);
                    let new_len = start + len - new_start;
                    assert!(new_start >= start);
                    assert!(new_len <= len);

                    next_ranges.push((r.dst_start + off, len - new_len));
                    start = new_start;
                    len = new_len;
                }
                if len == 0 {
                    break;
                }
            }
        }
        cur_ranges.clear();
        cur_ranges.extend(next_ranges.drain(..));
    }
    let mut min_pairs_loc = usize::MAX;
    for pair in cur_ranges.iter() {
        min_pairs_loc = usize::min(min_pairs_loc, pair.0);
    }

    (format!("{min_single_loc}"), format!("{min_pairs_loc}"))
}
