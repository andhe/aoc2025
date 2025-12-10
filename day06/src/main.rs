use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    println!("INIT PART 2");

    let mut part2_sum = 0;

    let grid: Vec<Vec<char>> = String::from_utf8(fs::read("input.txt")?).unwrap()
    .lines()
    .map(|line| line.chars().collect())
    .collect();

    let mut col_numbers: Vec<i64> = Vec::new();

    for idx in (0..grid[0].len()).rev() {
        let mut col_val = 0;
        let mut operator = None;

        //for row in grid.iter().take(grid.len()-1) {
        for row in grid.iter() {
            let c = row[idx];

            // check if we have an operator.
            operator = match c {
                '*' => Some('*'),
                '+' => Some('+'),
                _ => None,
            };
            if operator != None {
                break;
            }

            let optval = match c {
                ' ' => None,
                '*' => None,
                '+' => None,
                _ => Some(c.to_digit(10).unwrap() as i64),
            };

            if optval != None {
                let val = optval.unwrap();
                col_val = col_val * 10 + val;
            }
        }

        if col_val != 0 {
            println!("col_val={}", col_val);
            col_numbers.push(col_val);
        }

        if operator != None {
            let result = match operator.unwrap() {
                '*' => col_numbers.clone().into_iter().product::<i64>(),
                '+' => col_numbers.clone().into_iter().sum::<i64>(),
                _ => panic!("unknown operator"),
            };

            println!("Result: {}", result);

            part2_sum += result;

            col_numbers.clear();
        }
    }

    println!("Part 2 - total sum: {}", part2_sum);

    Ok(())
}

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

    part2();

    Ok(())
}
