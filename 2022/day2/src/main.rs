use anyhow::Result;
use std::{env, fs, str::FromStr};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for RPS {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissor),
            _ => Err(anyhow::format_err!("{s} is not a valid move")),
        }
    }
}

impl Into<i32> for RPS {
    fn into(self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissor => 3,
        }
    }
}

impl std::fmt::Display for RPS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RPS::Rock => write!(f, "rock"),
            RPS::Paper => write!(f, "paper"),
            RPS::Scissor => write!(f, "scissor"),
        }
    }
}

fn main() -> Result<()> {
    let p = env::current_dir()?;
    let data = fs::read_to_string(p.join("day2/input.txt"))?;
    let parsed: Vec<_> = data
        .trim()
        .split("\n")
        .into_iter()
        .map(|x| {
            x.split(" ")
                .filter_map(|y| y.parse::<RPS>().ok())
                .collect::<Vec<_>>()
        })
        .collect();

    let score = parsed.iter().fold(0, |acc, game| {
        let mut game_val: i32 = 0;
        let play_val: i32 = game[1].clone().into();
        if game.get(0) < game.get(1) {
            game_val = game_val + play_val + 6;
        } else if game.get(0) == game.get(1) {
            game_val = game_val + play_val + 3;
        } else {
            game_val = game_val + play_val;
        }
        acc + game_val
    });

    println!("part 1: {score}");

    Ok(())
}
