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

    fn find_bad(&self) -> Vec<i64> {
        let mut bad: Vec<i64> = vec![];

        for v in self.start..=self.stop {
            let s = v.to_string();
            let mid = s.len() / 2;

            if &s[..mid] == &s[mid..] {
                //println!("Range {:?} has repeated {}", self, s);
                bad.push(v);
            }
        }

        bad
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut badsum: i64 = 0;

    for linei in reader.lines() {
        let line = linei?;

        //println!("Line: {}", line);

        for range in line.split(',') {
            if let Ok(r) = Range::parse(&range) {
                //println!("Range: {:?}", r);

                let bad = r.find_bad();
                println!("range: {:?}, bad: {:?}", r, bad);
                badsum += bad.iter().sum::<i64>();
            }
        }

    }
    
    println!("Summary of bad numbers: {}", badsum);

    Ok(())
}
