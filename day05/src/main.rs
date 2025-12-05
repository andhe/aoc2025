use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
struct Range {
    start: i64,
    stop: i64,
}

impl Range {
    fn parse(rangestring: &str) -> Result<Range, String> {
        let (left, right) = rangestring.split_once('-')
            .ok_or("missing '-' in range string")?;
        
        //println!("Left: {}, Right: {}", left, right);

        let start: i64 = left.parse()
            .map_err(|e| format!("Invalid number '{}': {}", left, e))?;
        let stop: i64 = right.parse()
            .map_err(|e| format!("Invalid number '{}': {}", right, e))?;

        Ok(Range { start, stop })
    }

    fn contains(&self, val: i64) -> bool {
        //println!("DEBUG: val:{}, start:{}, stop:{}", val, self.start, self.stop);
        val >= self.start && val <= self.stop
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut mode = 0;
    let mut ranges = Vec::new();

    let mut spoiled = Vec::new();
    let mut fresh = Vec::new();

    for linei in reader.lines() {
        let line = linei?;

        //println!("Line: {}", line);

        if line == "" { mode += 1; continue; }

        if mode == 0 {
            if let Ok(r) = Range::parse(&line) {
                println!("Range: {:?}", r);

                ranges.push(r);
            }
        } else {
            let mut found: bool = false;
            let lineval:i64 = line.parse().unwrap();
            for r in &ranges {
                found = r.contains(lineval);
                if found { break; }
            }

            if found {
                println!("Value {} found in range.", lineval);
                fresh.push(lineval);
            } else {
                println!("Not in any range: {}", lineval);
                spoiled.push(lineval);
            }
        }

    }

    println!("Found {} spoiled and {} fresh.", spoiled.len(), fresh.len());

    Ok(())
}
