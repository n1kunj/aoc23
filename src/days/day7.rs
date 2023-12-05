use std::collections::BTreeMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: usize,
}

pub fn main(input: &str) -> (String, String) {
    let mut hands = Vec::<Hand>::new();
    let mut hands2 = Vec::<Hand>::new();
    for l in input.lines() {
        let mut es = l.split_whitespace();
        let hand = es.next().unwrap();
        let bid = es.next().unwrap().parse::<usize>().unwrap();
        assert!(hand.len() == 5);

        hands.push(|| -> Hand {
            fn parse(c: char) -> Card {
                match c {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'J' => Card::Jack,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    other => panic!("{other}"),
                }
            }

            let mut hand_cs = hand.chars();
            let mut next = || hand_cs.next().unwrap();
            let cards = [
                parse(next()),
                parse(next()),
                parse(next()),
                parse(next()),
                parse(next()),
            ];

            assert!(hand_cs.next().is_none());

            let mut counts = BTreeMap::<Card, usize>::new();
            for card in cards.iter() {
                counts
                    .entry(*card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            let mut fives = 0usize;
            let mut fours = 0usize;
            let mut threes = 0usize;
            let mut twos = 0usize;
            for (_, count) in counts.iter() {
                match count {
                    5 => fives += 1,
                    4 => fours += 1,
                    3 => threes += 1,
                    2 => twos += 1,
                    _ => (),
                }
            }
            let hand_type = match (fives, fours, threes, twos) {
                (1, 0, 0, 0) => HandType::FiveOfAKind,
                (0, 1, 0, 0) => HandType::FourOfAKind,
                (0, 0, 1, 1) => HandType::FullHouse,
                (0, 0, 1, 0) => HandType::ThreeOfAKind,
                (0, 0, 0, 2) => HandType::TwoPair,
                (0, 0, 0, 1) => HandType::OnePair,
                (_, _, _, _) => HandType::HighCard,
            };

            Hand {
                hand_type,
                cards,
                bid,
            }
        }());

        hands2.push(|| -> Hand {
            fn parse(c: char) -> Card {
                match c {
                    'A' => Card::Ace,
                    'K' => Card::King,
                    'Q' => Card::Queen,
                    'T' => Card::Ten,
                    '9' => Card::Nine,
                    '8' => Card::Eight,
                    '7' => Card::Seven,
                    '6' => Card::Six,
                    '5' => Card::Five,
                    '4' => Card::Four,
                    '3' => Card::Three,
                    '2' => Card::Two,
                    'J' => Card::Joker,
                    other => panic!("{other}"),
                }
            }

            let mut hand_cs = hand.chars();
            let mut next = || hand_cs.next().unwrap();
            let cards = [
                parse(next()),
                parse(next()),
                parse(next()),
                parse(next()),
                parse(next()),
            ];
            assert!(hand_cs.next().is_none());

            let mut counts = BTreeMap::<Card, usize>::new();
            for card in cards.iter() {
                counts
                    .entry(*card)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            let mut fives = 0usize;
            let mut fours = 0usize;
            let mut threes = 0usize;
            let mut twos = 0usize;
            let mut jokers = 0usize;
            for (card, count) in counts.iter() {
                if matches!(card, Card::Joker) {
                    jokers = *count;
                } else {
                    match count {
                        5 => fives += 1,
                        4 => fours += 1,
                        3 => threes += 1,
                        2 => twos += 1,
                        _ => (),
                    }
                }
            }
            let hand_type = match (fives, fours, threes, twos, jokers) {
                (1, 0, 0, 0, 0) => HandType::FiveOfAKind,
                (0, 1, 0, 0, 1) => HandType::FiveOfAKind,
                (0, 0, 1, 0, 2) => HandType::FiveOfAKind,
                (0, 0, 0, 1, 3) => HandType::FiveOfAKind,
                (0, 0, 0, 0, 4) => HandType::FiveOfAKind,
                (0, 0, 0, 0, 5) => HandType::FiveOfAKind,
                (0, 1, 0, 0, 0) => HandType::FourOfAKind,
                (0, 0, 1, 0, 1) => HandType::FourOfAKind,
                (0, 0, 0, 1, 2) => HandType::FourOfAKind,
                (0, 0, 0, 0, 3) => HandType::FourOfAKind,
                (0, 0, 1, 1, 0) => HandType::FullHouse,
                (0, 0, 0, 2, 1) => HandType::FullHouse,
                (0, 0, 1, 0, 0) => HandType::ThreeOfAKind,
                (0, 0, 0, 1, 1) => HandType::ThreeOfAKind,
                (0, 0, 0, 0, 2) => HandType::ThreeOfAKind,
                (0, 0, 0, 2, 0) => HandType::TwoPair,
                (0, 0, 0, 1, 0) => HandType::OnePair,
                (0, 0, 0, 0, 1) => HandType::OnePair,
                (_, _, _, _, _) => HandType::HighCard,
            };

            Hand {
                hand_type,
                cards,
                bid,
            }
        }());
    }

    hands.sort();
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0usize, |a, (i, h)| a + (i + 1) * h.bid);

    hands2.sort();
    let total_winnings2 = hands2
        .iter()
        .enumerate()
        .fold(0usize, |a, (i, h)| a + (i + 1) * h.bid);

    (format!("{total_winnings}"), format!("{total_winnings2}"))
}
