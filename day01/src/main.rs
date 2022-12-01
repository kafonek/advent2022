// https://adventofcode.com/2022/day/1
// 1. Find the largest sum of numbers
// 2. Find the sum of the three largest sum of numbers
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut counts: Vec<usize> = Vec::new();
    let reader = BufReader::new(File::open("data.txt").expect("cannot read data.txt"));
    let mut count: usize = 0;
    for line in reader.lines() {
        let text = line.unwrap();
        if text.trim().is_empty() {
            counts.push(count);
            count = 0;
        } else {
            count += text.parse::<usize>().unwrap();
        }
    }

    // sort descending (largest counts first)
    counts.sort_by(|a, b| b.cmp(a));

    // solution for #1
    println!("{}", counts.first().unwrap().to_string());

    // solution for #2
    println!("{}", &counts[..3].iter().sum::<usize>().to_string());
}
