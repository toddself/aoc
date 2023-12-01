use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let data = fs::read_to_string("input.txt")?;

    let parsed1: Vec<_> = data
        .split("\n")
        .into_iter()
        .map(|x| {
            let mut nums = vec![];
            x.chars().into_iter().for_each(|c| {
                if let Some(num) = c.to_digit(10) {
                    nums.push(num);
                };
            });
            (nums.first().unwrap_or(&0) * 10) + nums.last().unwrap_or(&0)
        })
        .collect();

    let part1 = parsed1.iter().fold(0, |a, x| a + x);
    println!("part 1: {part1}");

    let parsed2: Vec<_> = data
        .split("\n")
        .into_iter()
        .map(|x| {
            let mut nums = vec![];
            let mut collector = String::new();
            x.chars().into_iter().for_each(|c| {
                if let Some(num) = c.to_digit(10) {
                    nums.push(num);
                    collector.clear();
                } else {
                    collector.push(c);
                    if collector.contains("one") {
                        nums.push(1);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    } else if collector.contains("two") {
                        nums.push(2);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    } else if collector.contains("three") {
                        nums.push(3);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    } else if collector.contains("four") {
                        nums.push(4);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    } else if collector.contains("five") {
                        nums.push(5);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    } else if collector.contains("six") {
                        nums.push(6);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    } else if collector.contains("seven") {
                        nums.push(7);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    } else if collector.contains("eight") {
                        nums.push(8);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    } else if collector.contains("nine") {
                        nums.push(9);
                        let last = collector.pop().clone().unwrap_or_default();
                        collector.clear();
                        collector.push(last);
                    }
                }
            });
            (nums.first().unwrap_or(&0) * 10) + nums.last().unwrap_or(&0)
        })
        .collect();

    let part2 = parsed2.iter().fold(0, |a, x| a + x);
    println!("part 2: {part2}");

    Ok(())
}
