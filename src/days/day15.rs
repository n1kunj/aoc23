pub fn main(input: &str) -> (String, String) {
    let mut lines = input.lines();
    let l = lines.next();

    fn hash_str(s: &str) -> usize {
        let mut cur = 0usize;
        for c in s.chars() {
            assert!(c.is_ascii());
            let v = c as usize;
            cur += v;
            cur *= 17;
            cur %= 256;
        }
        cur
    }

    let strings = l
        .unwrap()
        .split(",")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    let hashed_strs = strings.iter().map(|s| hash_str(s)).collect::<Vec<_>>();
    assert!(lines.next().is_none());

    let hashed_sum = hashed_strs.iter().sum::<usize>();

    let mut boxes = Vec::<Vec<(String, usize)>>::new();
    boxes.resize_with(256, || Vec::<(String, usize)>::new());

    for s in strings.iter() {
        if s.contains("=") {
            let mut ss = s.split("=");
            let label = ss.next().unwrap();
            let hash = hash_str(label);
            let fl = ss.next().unwrap().parse::<usize>().unwrap();

            let b = &mut boxes[hash];
            let mut replaced = false;
            for l in b.iter_mut() {
                if l.0 == label {
                    l.1 = fl;
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                b.push((label.to_owned(), fl));
            }
        } else {
            assert!(s.ends_with("-"));
            let label = &s[0..s.len() - 1];
            let hash = hash_str(label);

            let b = &mut boxes[hash];
            b.retain(|l| l.0 != label);
        }
    }

    let mut focusing_power = 0usize;
    for (b_idx, b) in boxes.iter().enumerate() {
        for (slot_idx, slot) in b.iter().enumerate() {
            focusing_power += (1 + b_idx) * (1 + slot_idx) * slot.1;
        }
    }

    (format!("{hashed_sum}"), format!("{focusing_power}"))
}
