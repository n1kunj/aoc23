use regex::Regex;

#[derive(Debug)]
struct Subset {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    game_id: usize,
    subsets: Vec<Subset>,
}

pub fn main(input: &str) -> (String, String) {
    let game_re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let grab_re = Regex::new(r"(\d+) (.+)").unwrap();
    let mut games = Vec::<Game>::new();
    for l in input.lines() {
        for game in game_re.captures_iter(l) {
            let game_id = game.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let rounds = game.get(2).unwrap().as_str();

            let mut subsets = Vec::<Subset>::new();

            for round in rounds.split("; ") {
                let mut red = 0usize;
                let mut green = 0usize;
                let mut blue = 0usize;
                for grab in round.split(", ") {
                    for cubes in grab_re.captures_iter(grab) {
                        let count = cubes.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        match cubes.get(2).unwrap().as_str() {
                            "red" => red = count,
                            "green" => green = count,
                            "blue" => blue = count,
                            other => panic!("{other}"),
                        };
                    }
                }
                subsets.push(Subset { red, green, blue })
            }
            games.push(Game { game_id, subsets })
        }
    }

    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let mut possible_game_id_sum = 0usize;

    for g in games.iter() {
        let mut any_impossible = false;
        for s in g.subsets.iter() {
            let is_possible = s.red <= MAX_RED && s.green <= MAX_GREEN && s.blue <= MAX_BLUE;
            if !is_possible {
                any_impossible = true;
            }
        }
        if !any_impossible {
            possible_game_id_sum += g.game_id;
        }
    }

    let mut game_power_sum = 0usize;

    for g in games.iter() {
        let mut min_red = 0usize;
        let mut min_green = 0usize;
        let mut min_blue = 0usize;
        for s in g.subsets.iter() {
            min_red = usize::max(min_red, s.red);
            min_green = usize::max(min_green, s.green);
            min_blue = usize::max(min_blue, s.blue);
        }
        let game_power = min_red * min_green * min_blue;
        game_power_sum += game_power;
    }

    (format!("{possible_game_id_sum}"), format!("{game_power_sum}"))
}
