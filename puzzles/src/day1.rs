use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::cmp::max;

fn part1() -> io::Result<()> {
    let file = File::open("./src/day1.txt")?;
    let reader = BufReader::new(file);
    let mut elves = Vec::<i32>::new();

    let mut idx: usize = 0;
    let mut next_elf_please = true;

    for line in reader.lines() {
        let cal: Result<i32,_> = line?.parse::<i32>();
        match cal {
            Ok(res) => 
                if next_elf_please {
                    elves.push(res);
                    next_elf_please = false;
                } else {
                    elves[idx] += res;
                },
            _ => {idx += 1; next_elf_please = true},
        };
    }

    let mut maxcal = 0;
    for elf in elves {
        maxcal = max(maxcal, elf);
    }

    println!("Max calories: {}", maxcal);

    Ok(())
}

fn part2() -> io::Result<()> {
    Ok(())
}

pub fn run_all () {
    println!("Day 1 Part 1:");
    let _err = part1();
    println!("Day 1 Part 2:");
    let _err = part2();
}