use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid: Vec<Vec<String>> = reader.lines()
        .map(|line_result| {
            let line = line_result.unwrap();
            line.split_whitespace()
                .map(|s| s.to_string())
                .collect()
        })
    .collect();

    let operands = &grid[grid.len()-1];
    println!("{:?}", operands);

    //println!("{:?}", grid);

    let mut total_sum = 0;

    for idx in 0..grid[0].len() {
        let col_values: Vec<i64> = grid.iter().take(grid.len()-1)
            .map(|row| row[idx].parse::<i64>().unwrap())
            .collect();

        let result = match operands[idx].chars().nth(0).unwrap() {
            '*' => col_values.iter().product::<i64>(),
            '+' => col_values.iter().sum::<i64>(),
            _ => panic!("Unsupported operator"),
        };
        println!("col[{}] = {}", idx, result);

        total_sum += result;
    }

    println!("Total sum: {}", total_sum);

    Ok(())
}
