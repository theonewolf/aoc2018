use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
#[derive(PartialEq)]
struct Claim {
    id: u64,
    x: u64,
    y: u64,
    w: u64,
    h: u64
}

impl Claim {
    fn from_string(raw: &str) -> Claim {
        let cleaned = raw.replace('#', "").replace('@', "").replace(':',"");
        let mut split = cleaned.split_whitespace();

        let id = split.next().unwrap().parse::<u64>().unwrap();

        let mut splitoff = split.next().unwrap().split(',');
        let x = splitoff.next().unwrap().parse::<u64>().unwrap();
        let y = splitoff.next().unwrap().parse::<u64>().unwrap();

        let mut dimsplit = split.next().unwrap().split('x');
        let w = dimsplit.next().unwrap().parse::<u64>().unwrap();
        let h = dimsplit.next().unwrap().parse::<u64>().unwrap();

        Claim {
            id: id,
            x: x,
            y: y,
            w: w,
            h: h
        }
    }

    fn contains(&self, x: u64, y: u64) -> bool {
        x >= self.x &&
        y >= self.y &&
        x < self.x + self.w &&
        y < self.y + self.h
    }

    fn overlaps(&self, other: &Claim) -> bool {
        !(self.x + self.w < other.x ||
          other.x + other.w < self.x ||
          self.y + self.h < other.y ||
          other.y + other.h < self.y)
    }
}

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

fn parse_input(input: &String) -> Vec<Claim> {
    let mut claims: Vec<Claim> = vec![];

    for s in input.lines() {
        if !s.is_empty() {
            claims.push(Claim::from_string(s));
        }
    }

    return claims;
}

fn test_grid(claims: &mut Vec<Claim>, width: u64, height: u64) -> u64 {
    let mut count: u64 = 0;

    for x in 0..width {
        for y in 0..height {
            let mut contained = false;

            for c in claims.iter_mut() {
                if c.contains(x, y) && contained { count += 1; break; }
                if c.contains(x, y) { contained = true; }
            }
        }
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = read_input(&args[1]);
    let mut claims = parse_input(&s);

    println!("\nPart1\n--------------\n{:?}", test_grid(&mut claims, 1000, 1000));

    for c in &claims {
        let mut overlaps = false;

        for c2 in &claims {
            if c != c2 && c.overlaps(c2) { overlaps = true; break; }
        }

        if !overlaps {
            println!("\nPart2\n--------------\n{:?}", c.id);
        }
    }
}
