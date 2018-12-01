use std::collections::HashSet;
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

fn parse_input(input: &String) -> Vec<i64> {
    let mut steps: Vec<i64> = vec![];

    for s in input.split('\n') {
        match s.parse::<i64>() {
            Ok(n) => steps.push(n),
            Err(why) => println!("Error parsing '{}': {}", s, why)
        }
    }

    return steps;
}

fn frequency(steps: &Vec<i64>) -> (i64, i64) {
    let mut freq: i64 = 0;
    let mut repeat: i64 = 0;
    let mut found: bool = false;
    let mut freqs = HashSet::new();

    for n in steps.iter() {
        freq += n;

        if !found {
            if freqs.contains(&freq) {
                repeat = freq;
                found = true;
            }
        }

        freqs.insert(freq);
    }

    let freq_final = freq;

    while !found {
        for n in steps.iter() {
            freq += n;

            if !found {
                if freqs.contains(&freq) {
                    repeat = freq;
                    found = true;
                }
            }

            freqs.insert(freq);
        }
    }

    return (freq_final, repeat);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = read_input(&args[1]);
    let steps = parse_input(&s);

    let (freq, repeat) = frequency(&steps);
    println!("\nPart1\n---------------------\n{}", freq);
    println!("\nPart2\n---------------------\n{}", repeat);
}
