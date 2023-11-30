pub fn main(input: &str) -> (String, String) {
    let mut elf_cals = Vec::<usize>::new();
    let mut cur_elf_cals = 0usize;

    for l in input.lines() {
        match l {
            "" => {
                elf_cals.push(cur_elf_cals);
                cur_elf_cals = 0;
            }
            v => {
                cur_elf_cals += v.parse::<usize>().unwrap();
            }
        }
    }
    if cur_elf_cals != 0 {
        elf_cals.push(cur_elf_cals);
    }

    elf_cals.sort_by(|a, b| b.cmp(a));

    let most_cals = elf_cals.first().unwrap();
    let top_three_cals: usize = elf_cals[0..3].iter().sum();

    (format!("{most_cals}"), format!("{top_three_cals}"))
}
