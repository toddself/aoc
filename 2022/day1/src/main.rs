use anyhow::Result;
use std::{env, fs};

fn main() -> Result<()> {
    let p = env::current_dir()?;
    let data = fs::read_to_string(p.join("day1/input.txt"))?;

    let mut sums: Vec<_> = data
        .split("\n\n")
        .into_iter()
        .map(|x| {
            x.split("\n")
                .into_iter()
                .filter_map(|s| s.parse().ok())
                .fold(0, |a, x: i64| a + x)
        })
        .collect::<Vec<_>>();
    sums.sort();
    sums.reverse();

    println!("part 1: {}", sums.get(0).unwrap_or(&0));

    println!("part 2: {:?}", sums[0..3].iter().sum::<i64>());

    Ok(())
}
