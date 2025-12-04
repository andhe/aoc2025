use std::fs;

fn neighbors(x: isize, y: isize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    let dirs = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    for (dx, dy) in dirs.iter() {
        let nx = x + dx;
        let ny = y + dy;

        if nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height {
            result.push((nx as usize, ny as usize));
        }
    }

    result
}

fn is_movable(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    // surrounding positions
    let height = grid.len();
    let width = grid[0].len();

    let n = neighbors(x as isize, y as isize, width, height);

    // count surrounding rolls
    let mut rolls = 0;
    for (px,py) in n.iter() {
        if grid[*py][*px] == '@' {
            rolls += 1;
        }
    }

    if rolls < 4 {
        return true
    }

    return false
}

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    // Each line becomes a Vec<char>; all lines form Vec<Vec<char>>
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Print for debugging
    for row in &grid {
        println!("{:?}", row);
    }

    let height = grid.len();
    let width = grid[0].len();

    println!("Height {} x Width {}", height, width);

    let mut movable_grid = grid.clone();
    let mut movable: Vec<(usize,usize)> = Vec::new();

    // find positions
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '@' {
                println!("Roll at: x: {}, y: {}", x, y);
                if is_movable(&grid, x, y) {
                    println!("{}x{} is movable!", x, y);
                    movable_grid[y][x] = 'x';
                    movable.push((x,y));
                }
            }
        }
    }

    // Print result
    for row in &movable_grid {
        println!("{:?}", row);
    }
    println!("Movable rolls: {}", movable.len());

    Ok(())
}
