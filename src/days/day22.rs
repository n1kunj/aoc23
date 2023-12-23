use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct End {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Copy)]
struct Block {
    a: End,
    b: End,
}

impl Block {
    fn dx(&self) -> usize {
        self.b.x - self.a.x + 1
    }
    fn dy(&self) -> usize {
        self.b.y - self.a.y + 1
    }
    fn dz(&self) -> usize {
        self.b.z - self.a.z + 1
    }
    fn x(&self) -> usize {
        self.a.x
    }
    fn y(&self) -> usize {
        self.a.y
    }
    fn z(&self) -> usize {
        self.a.z
    }

    fn intersects(&self, other: &Self) -> bool {
        let ranges_overlap = |a: usize, ad: usize, b: usize, bd: usize| -> bool {
            if a + ad <= b {
                return false;
            } else if b + bd <= a {
                return false;
            }
            return true;
        };
        let xo = ranges_overlap(self.x(), self.dx(), other.x(), other.dx());
        let yo = ranges_overlap(self.y(), self.dy(), other.y(), other.dy());
        let zo = ranges_overlap(self.z(), self.dz(), other.z(), other.dz());
        return xo && yo && zo;
    }

    fn lower_by(&self, lz: usize) -> Block {
        assert!(lz < self.z());
        let mut a = self.a;
        let mut b = self.b;
        a.z -= lz;
        b.z -= lz;
        Block { a, b }
    }
}

pub fn main(input: &str) -> (String, String) {
    fn parse_end(s: &str) -> End {
        let mut it = s.split(",");
        let mut parse = || it.next().unwrap().parse::<usize>().unwrap();
        let x = parse();
        let y = parse();
        let z = parse();
        assert!(it.next().is_none());
        End { x, y, z }
    }

    let mut blocks = Vec::<Block>::new();
    for l in input.lines() {
        let mut s = l.split("~");
        let a = parse_end(s.next().unwrap());
        let b = parse_end(s.next().unwrap());
        assert!(s.next().is_none());
        blocks.push(Block { a, b });
    }

    blocks.sort_by_key(|b| b.z());

    let blocks = blocks;

    let mut settled_blocks = Vec::<Block>::new();

    for block in blocks.iter() {
        assert!(blocks.iter().filter(|b| b.intersects(&block)).count() == 1);
        assert!(settled_blocks.iter().all(|sb| !sb.intersects(&block)));
        let z = block.z();
        let mut nz = block.z();
        while nz > 1 {
            nz -= 1;
            let nb = block.lower_by(z - nz);
            if settled_blocks.iter().any(|sb| sb.intersects(&nb)) {
                nz += 1;
                break;
            }
        }
        settled_blocks.push(block.lower_by(z - nz));
    }

    let mut can_be_removed_count = 0usize;
    for candidate in settled_blocks.iter() {
        let mut any_moved = false;
        for b in settled_blocks.iter() {
            // Ignore the candidate block.
            if std::ptr::eq(b, candidate) {
                continue;
            }
            // Other block touches the ground, can't be supported by candidate.
            if b.z() == 1 {
                continue;
            }

            let nb = b.lower_by(1);

            let mut any_intersect = false;
            for b2 in settled_blocks.iter() {
                // Don't compare against ourself.
                if std::ptr::eq(b2, b) {
                    continue;
                }
                // Don't compare against the candidate.
                if std::ptr::eq(b2, candidate) {
                    continue;
                }
                if nb.intersects(b2) {
                    any_intersect = true;
                    break;
                }
            }
            if !any_intersect {
                any_moved = true;
                break;
            }
        }
        if !any_moved {
            can_be_removed_count += 1;
        }
    }

    let mut supported_by_indices = blocks
        .iter()
        .map(|_| Vec::<usize>::new())
        .collect::<Vec<_>>();

    // For each block, move it down by one and see which blocks intersect it. Those are supports.
    for (ib, b) in settled_blocks.iter().enumerate() {
        if b.z() == 1 {
            continue;
        }
        let nb: Block = b.lower_by(1);
        for (ib2, b2) in settled_blocks.iter().enumerate() {
            // Don't compare against ourself.
            if std::ptr::eq(b2, b) {
                continue;
            }
            if nb.intersects(b2) {
                supported_by_indices[ib].push(ib2);
            }
        }
    }

    let mut causes_to_fall = Vec::<usize>::new();

    for i in 0..settled_blocks.len() {
        let mut remove_is = HashSet::<usize>::new();
        remove_is.insert(i);
        let mut any_added = true;
        while any_added {
            any_added = false;
            for j in 0..settled_blocks.len() {
                if supported_by_indices[j].is_empty() {
                    continue;
                }
                if remove_is.contains(&j) {
                    continue;
                }
                if supported_by_indices[j]
                    .iter()
                    .all(|n| remove_is.contains(n))
                {
                    remove_is.insert(j);
                    any_added = true;
                }
            }
        }
        causes_to_fall.push(remove_is.len() - 1);
    }

    let sum_falling = causes_to_fall.iter().sum::<usize>();

    (format!("{can_be_removed_count}"), format!("{sum_falling}"))
}
