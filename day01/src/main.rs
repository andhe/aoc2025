use std::fs::File;
use std::io::{self, BufRead, BufReader};



#[derive(Debug)]
struct Command {
    dir: char,
    value: i32,
}


impl Command {
    fn parse(line: &str) -> Result<Command, String> {
        let mut chars = line.chars();

        // Get first character (L or R)
        let dir = chars
            .next()
            .ok_or_else(|| "Line is empty".to_string())?;

        if dir != 'L' && dir != 'R' {
            return Err(format!("Invalid direction '{}'", dir));
        }

        // Get the rest of the line
        let rest: String = chars.collect();

        // Parse number
        let value: i32 = rest
            .parse()
            .map_err(|e| format!("Invalid number '{}': {}", rest, e))?;

        Ok(Command { dir, value })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut dial: i32 = 50;
    let mut zero_count: i32 = 0;

    // Read it line by line
    for line_result in reader.lines() {
        let line = line_result?;
        println!("{}", line);

        if let Ok(cmd) = Command::parse(&line) {
            println!("{:?}", cmd);
            if cmd.dir == 'R' {
                dial += cmd.value%100;
                if dial > 99 {
                    dial -= 100;
                }
            } else {
                dial -= cmd.value%100;
                if dial < 0 {
                    dial += 100;
                }
            }

            println!("Dial: {}", dial);

            if dial == 0 {
                zero_count += 1;
            }
        }

    }

    println!("Ended up on zero: {} times", zero_count);

    Ok(())
}
