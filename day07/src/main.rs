use std::fs;

fn find_s(grid: &Vec<Vec<char>>) -> (usize, usize) {
    // assume S is always found in grid.

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 'S' {
                return (row, col);
            }
        }
    }

    panic!("find_s did not find any S in grid");
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}

fn beam_down_from(grid: &mut Vec<Vec<char>>, startrow: usize, col: usize) -> u32 {
    // Note: count number of splits.
    let mut num_splits = 0;

    println!("Beaming down from row:{}, col:{}...", startrow, col);
    print_grid(grid);

    // beam always moves downwards.... check row below start position.
    for row in startrow+1..grid.len() {
        println!("DEBUG: checking row:{}, col:{}", row, col);
        println!("DEBUG: grid[{}][{}] = {}", row, col, grid[row][col]);
        match grid[row][col] {
            '.' => {
                grid[row][col] = '|';
            },
            '^' => {
                // until it hits a splitter,
                num_splits += 1;
                // then it stops and continues on ...
                // ... left (unless we're at the left border)...
                if col > 0 {
                    grid[row][col-1] = '|';
                    num_splits += beam_down_from(grid, row, col-1);
                }
                // and right side.
                if col+1 < grid[0].len() {
                    grid[row][col+1] = '|';
                    num_splits += beam_down_from(grid, row, col+1);
                }

                return num_splits;
            },
            '|' => {
                // we where already here (in a previous split beam)
                return num_splits;
            },
            _ => panic!("Unknown character encountered in grid"),
        };
    }
    
    print_grid(grid);
    return num_splits;
}


fn part1(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("===== PART 1 =====");

    let mut grid: Vec<Vec<char>> = String::from_utf8(fs::read(filename)?).unwrap()
    .lines()
    .map(|line| line.chars().collect())
    .collect();

    let (startrow, col) = find_s(&grid);

    assert_eq!(grid[startrow][col], 'S');
    
    let total_splits = beam_down_from(&mut grid, startrow, col);

    println!("Total splits: {}", total_splits);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "input.txt";
    part1(filename)?;
    //part2(filename)?;

    Ok(())
}
