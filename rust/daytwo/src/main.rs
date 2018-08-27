use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Load spreadsheet
    let mut f = File::open("sheet.csv").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    for line in contents.lines() {
        let numbers = line.split_whitespace();
        let largest = numbers
            .clone()
            .map(|i| i.parse::<i32>().unwrap())
            .max()
            .unwrap();
        let smallest = numbers
            .clone()
            .map(|i| i.parse::<i32>().unwrap())
            .min()
            .unwrap();
        println!("{} {}", largest, smallest);
    }

    // Find largest and smallest numbers in each row
    // Sum them together
}
