mod days;

use days::DAYS;

use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    name: String,
    input: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut f: Option<&fn(&str) -> (String, String)> = None;

    for day in DAYS {
        if day.0 == args.name {
            f = Some(&day.1);
            break;
        }
    }

    let f = f.ok_or(format!("Unknown name {}", args.name))?;

    let path_name = format!("./inputs/{}/{}.txt", args.name, args.input);
    let ipath = Path::new(&path_name);
    let input = std::fs::read_to_string(ipath).unwrap();

    let (a, b) = f(&input);

    println!("First: {a}\nSecond: {b}");

    Ok(())
}
