use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_numbers<P>(filepath: P) -> Vec<i32>
where
    P: AsRef<Path>,
{
    let file = File::open(filepath).expect("Invalid data file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect()
}

fn sum(X: &[i32]) -> i32 {
    let mut total = 0;
    for x in X {
        total += x;
    }
    total
}

fn main() {
    let numbers = read_numbers("./input");

    for a in 0..numbers.len() {
        for b in (a + 1)..numbers.len() {
            for c in (b + 1)..numbers.len() {
                let x = numbers[a];
                let y = numbers[b];
                let z = numbers[c];
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                }
            }
        }
    }
}
