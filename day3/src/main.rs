use anyhow::Result;
use std::{env, fs};

fn main() -> Result<()> {
    let p = env::current_dir()?;
    let data = fs::read_to_string(p.join("day3/part1.txt"))?;

    let mut sum = 0;
    let matrix = data.trim().split("\n").map(|x | x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    matrix.iter().for_each(|row| println!("{:?}", row));

    let mut curr_row: usize = 0;
    matrix.iter().for_each(|row| {
        let mut curr_col: usize = 0;
        let mut collector = String::new();
        row.iter().for_each(|col| {
            if col.is_digit(10) {
                collector.push(col.clone());
            } else {
                if collector.len() >  0 {
                    let col_start = match curr_col - collector.len() {
                        0 => 0,
                        _ => curr_col - collector.len() - 1
                    };
                    let col_end = curr_col + 1;
                    for i in col_start .. col_end {
                        let row_start = if curr_row == 0 {
                            0
                        } else {
                            curr_row - 1
                        };

                        let row_end = curr_row + 1;
                        for j in row_start .. row_end {
                            let c = match matrix.get(j as usize) {
                                Some(v) => *v.get(i as usize).unwrap_or(&' '),
                                None => ' ',
                            };
                            if !c.is_digit(10) && c != '.' && c != ' ' {
                                println!("adding {collector} because {c} is a symbol");
                                sum += collector.parse().unwrap_or(0);
                            }
                        }
                    }
                }
                collector.clear();
            }
            curr_col += 1;
        });
        curr_row += 1;
    });

    println!("part 1: {sum}");
    
    Ok(())
}
