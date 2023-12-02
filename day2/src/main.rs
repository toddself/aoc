use anyhow::Result;
use std::{env, fs};

fn main() -> Result<()> {
    let p = env::current_dir()?;
    let data = fs::read_to_string(p.join("day2/input.txt"))?;
    let mut games = 0;
    let mut power: i64 = 0;
    data.trim().split("\n").into_iter().for_each(|game| {
        let (game_info, game_data) = game.split_once(':').unwrap_or(("", ""));
        let game_id = game_info
            .split_once(' ')
            .unwrap_or(("", ""))
            .1
            .parse()
            .unwrap_or(0);
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        game_data
            .split(";")
            .into_iter()
            .map(|x| x.trim())
            .for_each(|rounds| {
                rounds.trim().split(",").into_iter().for_each(|cube| {
                    let (count, color) = cube.trim().split_once(' ').unwrap_or(("", ""));
                    let count = count.trim().parse().unwrap_or(0);
                    match color.trim() {
                        "red" => red = red.max(count),
                        "blue" => blue = blue.max(count),
                        "green" => green = green.max(count),
                        _ => (),
                    };
                });
            });

        if red <= 12 && green <= 13 && blue <= 14 {
            println!("game id {game_id} red {red} blue {blue} green {green} {game_data}");
            games += game_id;
        }
        power += red * blue * green;
    });

    println!("part 1: {games}");
    println!("part 2: {power}");

    Ok(())
}
