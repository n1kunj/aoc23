use std::collections::BTreeSet;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
struct PartNum {
    y: usize,
    x: usize,
    val: usize,
}

#[derive(Debug)]
enum Entry {
    Number(char, PartNum),
    Symbol(char),
    Nothing,
}

#[derive(Debug)]
struct Row {
    entries: Vec<Entry>,
}

#[derive(Debug)]
struct Schematic {
    rows: Vec<Row>,
}

pub fn main(input: &str) -> (String, String) {
    let mut rows = Vec::<Row>::new();

    for (row_idx, l) in input.lines().enumerate() {
        let mut entries = Vec::<Entry>::new();
        let mut numbuf: String = String::new();
        for c in l.chars() {
            let mut was_num = false;
            match c {
                '.' => {
                    entries.push(Entry::Nothing);
                }
                c if c.is_numeric() => {
                    was_num = true;
                    numbuf.push(c);
                }
                other => entries.push(Entry::Symbol(other)),
            }
            if !was_num {
                if !numbuf.is_empty() {
                    let val = numbuf.parse::<usize>().unwrap();
                    let prev = entries.pop().unwrap();
                    let x = entries.len();

                    for c in numbuf.chars() {
                        entries.push(Entry::Number(c, PartNum { x, y: row_idx, val }));
                    }
                    entries.push(prev);
                    numbuf.clear();
                }
            }
        }
        if !numbuf.is_empty() {
            let val = numbuf.parse::<usize>().unwrap();
            let x = entries.len();
            for c in numbuf.chars() {
                entries.push(Entry::Number(c, PartNum { x, y: row_idx, val }));
            }
        }

        rows.push(Row { entries });
    }
    let s = Schematic { rows };

    let mut part_numbers = BTreeSet::<PartNum>::new();
    for (row_idx, row) in s.rows.iter().enumerate() {
        for (col_idx, entry) in row.entries.iter().enumerate() {
            if let Entry::Number(_, v) = entry {
                for x in col_idx.saturating_sub(1)..=col_idx.saturating_add(1) {
                    for y in row_idx.saturating_sub(1)..=row_idx.saturating_add(1) {
                        if let Some(other_row) = s.rows.get(y) {
                            if let Some(other_entry) = other_row.entries.get(x) {
                                if matches!(other_entry, Entry::Symbol(_)) {
                                    part_numbers.insert(*v);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let part_nums_summed: usize = part_numbers.iter().map(|x| -> usize { x.val }).sum();

    let mut summed_ratio = 0usize;
    for (row_idx, row) in s.rows.iter().enumerate() {
        for (col_idx, entry) in row.entries.iter().enumerate() {
            if let Entry::Symbol('*') = entry {
                let mut part_numbers = BTreeSet::<PartNum>::new();
                for x in col_idx.saturating_sub(1)..=col_idx.saturating_add(1) {
                    for y in row_idx.saturating_sub(1)..=row_idx.saturating_add(1) {
                        if let Some(other_row) = s.rows.get(y) {
                            if let Some(other_entry) = other_row.entries.get(x) {
                                if let Entry::Number(_, v) = other_entry {
                                    part_numbers.insert(*v);
                                }
                            }
                        }
                    }
                }

                if part_numbers.len() == 2 {
                    let a = part_numbers.first().unwrap().val;
                    let b = part_numbers.last().unwrap().val;
                    let ratio = a * b;
                    summed_ratio += ratio;
                }
            }
        }
    }

    (format!("{part_nums_summed}"), format!("{summed_ratio}"))
}
