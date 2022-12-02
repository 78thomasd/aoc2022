use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::BinaryHeap;

fn part1() -> io::Result<BinaryHeap<i32>> {
    let file = File::open("./src/day1.txt").unwrap();
    let reader = BufReader::new(file);
    let mut elves = BinaryHeap::<i32>::new();

    let mut elf_total = 0;

    for line in reader.lines() {
        let cal: Result<i32,_> = line?.parse::<i32>();
        match cal {
            Ok(res) => elf_total += res,
            _ => {elves.push(elf_total); elf_total = 0},
        };
    }

    if elf_total > 0 {
        elves.push(elf_total);
    }

    println!("Max calories: {}", elves.peek().unwrap());

    Ok(elves)
}

fn part2(mut elves: BinaryHeap<i32>) -> io::Result<()> {
    let mut total = 0;
    for count in 0..3 {
        let val = elves.pop().unwrap();
        total += val;
        println!("{}: {}", count, val);
    }
    println!("Total: {}", total);

    Ok(())
}

pub fn run_all () {
    println!("Day 1 Part 1:");
    let res = part1().unwrap();
    println!("Day 1 Part 2:");
    let _err = part2(res);
}