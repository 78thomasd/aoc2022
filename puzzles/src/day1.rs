use std::collections::BinaryHeap;
use std::io::{self};
use crate::utils;

fn part1() -> io::Result<BinaryHeap<i32>> {
    /*
     * Not the best way to do it but I want to play with
     * different datastructures.
     */
    let mut elves = BinaryHeap::<i32>::new();

    let mut elf_total = 0;

    for line in utils::aoc_input_lines("day1.txt")? {
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
    for _count in 0..3 {
        // we lose these elves from the mutable as a side effect.
        let val = elves.pop().unwrap();
        total += val;
        //println!("{}: {}", _count, val);
    }
    println!("Top 3 Total: {}", total);

    Ok(())
}

pub fn run () {
    let res = part1().unwrap();
    let _err = part2(res);
}