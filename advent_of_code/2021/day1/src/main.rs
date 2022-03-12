use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn read_numbers<T: std::str::FromStr>() -> Vec<T> 
    where <T as std::str::FromStr>::Err: core::fmt::Debug {

    let reader = BufReader::new(File::open("input.txt").unwrap());
    
    let mut numbers = Vec::<T>::new();

    for line in reader.lines() {
        let val: T = line.unwrap().parse().unwrap();
        numbers.push(val);
    }

    numbers
}

fn main() -> std::io::Result<()> {
    let numbers = read_numbers::<u32>();

    println!("{}", numbers[0]);
    Ok(())
}
