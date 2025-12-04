use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_biggest(s: &str) -> (i32, usize) {
    let mut off: usize = 0;
    let mut biggest_byte: u8 = s.as_bytes()[0];

    for (i, b) in s.bytes().enumerate() {
        //println!("byte {}: {}", i, b);
        if i == 0 {
            biggest_byte = b;
            off = i;
        } else if b > biggest_byte {
            biggest_byte = b;
            off = i;
        }
    }

    let num: i32 = (biggest_byte - b'0') as i32;
    (num, off)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut bignum_sum: i32 = 0;
    let mut second_sum: i64 = 0;

    for linei in reader.lines() {
        let line = linei?;

        println!("Line: {}", line);

        // ========== part 1 ==========

        /*
        let (first_num, first_offset) = find_biggest(&line[..line.len()-1]);

        println!("First biggest num: {}, offset: {}", first_num, first_offset);

        let (second_num, second_offset) = find_biggest(&line[first_offset+1..]);

        println!("Second biggest num: {}, offset: {}", second_num, second_offset);

        let bignum = first_num * 10 + second_num;

        println!("Biggest number: {}", bignum);

        bignum_sum += bignum;
        */

        // ========== part 2 ==========
        let mut skipped = 0;
        let mut next_start = 0;
        let mut result: i64  = 0;
        let skip_limit = line.len() - 12;

        while skipped < skip_limit {
            let last_search: bool = result >= 10_000_000_000;

            // sliding window of skip_limit, unless last - all the way to the end.
            let end_offset = if last_search { line.len()-1 } else if skipped == 0 { next_start+skip_limit-1 } else { next_start+skip_limit-skipped };

            println!("Debug: searching {}..={}", next_start, end_offset);
            let search_part = &line[next_start..=end_offset];

            let (num, offset) = find_biggest(search_part);
            println!("Biggest num (in search part {}): {}, offset: {}", search_part, num, offset);

            skipped += offset;
            next_start += offset + 1;

            result = result * 10 + num as i64;

            println!("Debug: skipped:{}, result:{}", skipped, result);

            if last_search {
                break;
            }
        }

        let mut remainder = "";
        if skipped == skip_limit {
            remainder = &line[next_start..line.len()-skip_limit+skipped];
        }

        let res: i64 = (result.to_string() + remainder).parse().unwrap();
        println!("Result: {}", res);
        second_sum += res;
        println!("DebugResult: {}#{}", result, remainder);
    }

    // ========== part 1 ==========
    println!("Sum of biggest numbers: {}", bignum_sum);

    // ========== part 2 ==========
    println!("Second sum: {}", second_sum);
    
    Ok(())
}
