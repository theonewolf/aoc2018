use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_input(argpath: &String) -> String {
    let path = Path::new(argpath);
    let display = path.display();
    let mut s = String::new();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => println!("Successfully read {}", display),
    }

    return s;
}

fn parse_input(input: &String) -> Vec<String> {
    let mut steps: Vec<String> = vec![];

    for s in input.split('\n') {
        if !s.is_empty() {
            steps.push(s.to_string());
        }
    }

    return steps;
}

fn counts(s: &String) -> (bool, bool) {
    let mut counts: (bool, bool) = (false, false);
    let mut histogram: HashMap<char, i64> = HashMap::new();

    for c in s.chars() {
        let count = histogram.entry(c).or_insert(0);
        *count += 1;
    }

    for (_, count) in histogram {
        if !counts.0 && count == 2 {
            counts.0 = true;
        }

        if !counts.1 && count == 3 {
            counts.1 = true;
        }
    }

    return counts;
}

fn checksum(counts: &Vec<(bool, bool)>) -> i64 {
    let mut twos: i64 = 0;
    let mut threes: i64 = 0;

    for count in counts {
        if count.0 {
            twos += 1;
        }

        if count.1 {
            threes += 1;
        }
    }

    return twos * threes;
}

fn find_match(strings: Vec<String>) -> String {

    for s in &strings {
        for s2 in &strings {
            let mut diff = 0;
            let mut buffer: String = "".to_string();

            for i in 0..s.len() {
                if s.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {
                    diff += 1;
                } else {
                    buffer += &s.chars().nth(i).unwrap().to_string();
                }
                if diff > 1 { break; }
            }

            if diff == 1 { return buffer; }

        }
    }

    return "".to_string();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut parsed_barcodes: Vec<(bool, bool)> = vec![];
    let s = read_input(&args[1]);
    let barcodes = parse_input(&s);

    for b in &barcodes {
        parsed_barcodes.push(counts(&b));
    }
    println!("\nPart1\n-------------------\n{:?}", checksum(&parsed_barcodes));

    println!("\nPart 2\n------------------\n{:?}", find_match(barcodes));
}
