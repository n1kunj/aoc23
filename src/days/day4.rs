use regex::Regex;

struct Card {
    card_id: usize,
    matches: usize,
}

impl Card {
    fn new(card_id: usize, winning: Vec<usize>, my: Vec<usize>) -> Card {
        let mut matches = 0usize;
        for w in winning.iter() {
            for m in my.iter() {
                if w == m {
                    matches += 1;
                }
            }
        }

        Card { card_id, matches }
    }
}

pub fn main(input: &str) -> (String, String) {
    let card_re = Regex::new(r"Card\s*(\d+):\s*([\d ]+)\s*\|\s*([\d ]+)\s*$").unwrap();

    let mut cards = Vec::<Card>::new();
    for l in input.lines() {
        for c in card_re.captures_iter(l) {
            let card_id = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let winning_raw = c.get(2).unwrap().as_str();
            let my_raw = c.get(3).unwrap().as_str();
            let mut winning = Vec::<usize>::new();
            for w in winning_raw.split_whitespace() {
                winning.push(w.parse::<usize>().unwrap());
            }
            let mut my = Vec::<usize>::new();
            for m in my_raw.split_whitespace() {
                my.push(m.parse::<usize>().unwrap());
            }
            cards.push(Card::new(card_id, winning, my));
        }
    }

    let mut sum_points = 0usize;
    for c in cards.iter() {
        let points = match c.matches {
            0 => 0,
            x => 2usize.pow(x as u32 - 1),
        };
        sum_points += points;
    }

    let mut processed_cards = 0usize;
    let mut card_counts = cards.iter().map(|_| 1usize).collect::<Vec<usize>>();
    for (i, c) in cards.iter().enumerate() {
        let count = *card_counts.get(i).unwrap();
        processed_cards += count;
        for i in 0..c.matches {
            let other_c_idx = c.card_id + i;
            card_counts[other_c_idx] += count;
        }
    }

    (format!("{sum_points}"), format!("{processed_cards}"))
}
