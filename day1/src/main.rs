use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let data = fs::read_to_string("input.txt")?;
    let parsed = data.split("\n").into_iter().map(|x| {
        let mut nums = vec![];
        x.chars().into_iter().for_each(|c| {
           if let Some(num) = c.to_digit(10) {
               nums.push(num);
           };
        });
        let res = nums.get(0).unwrap_or(&0) + nums.last().unwrap_or(&0);
    });
    
    Ok(())
}
