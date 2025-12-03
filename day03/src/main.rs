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

    for linei in reader.lines() {
        let line = linei?;

        println!("Line: {}", line);


        let (first_num, first_offset) = find_biggest(&line[..line.len()-1]);

        println!("First biggest num: {}, offset: {}", first_num, first_offset);

        let (second_num, second_offset) = find_biggest(&line[first_offset+1..]);

        println!("Second biggest num: {}, offset: {}", second_num, second_offset);

        let bignum = first_num * 10 + second_num;

        println!("Biggest number: {}", bignum);

        bignum_sum += bignum;
    }

    println!("Sum of biggest numbers: {}", bignum_sum);
    
    Ok(())
}
