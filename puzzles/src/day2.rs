use regex::Regex;
use crate::utils;

fn score(opps: char, mine: char) -> i32 {
    let outcome = 3 * ((4 + (mine as i32 - 'X' as i32)
                                 - (opps as i32 - 'A' as i32)) % 3);
    let shape = mine as i32 - 'W' as i32;

    //println!("{} {}: {} {} {}", opps, mine, outcome, shape, outcome + shape);

    return outcome + shape
}

fn part1() {
    let mut total = 0;
    //Possibly overkill but I wanted to play with regex stuff.
    let re = Regex::new("([A-C]) ([X-Z])").unwrap();
    for line in utils::aoc_input_lines("day2.txt").unwrap() {
        let s = line.unwrap();
        let cap = re.captures(&s).unwrap();
        let opps = cap.get(1).unwrap().as_str().chars().next().unwrap();
        let mine = cap.get(2).unwrap().as_str().chars().next().unwrap();
        total += score(opps, mine);
    }
    println!("Part1 Total score: {}", total);
}

fn score2(opps: char, aim: char) -> i32 {
    let opp_shape = 1 + opps as i32 - 'A' as i32;
    let mine_shape = 1 + (opp_shape + 2 + (aim as i32 - 'Y' as i32)) % 3;
    let outcome = 3 * ((4 + mine_shape - opp_shape) % 3); 

    //println!("{} {}: {} {} {}", opps, aim, opp_shape, mine_shape, outcome);

    return mine_shape + outcome
}


fn part2() {
    let mut total = 0;
    //Possibly overkill but I wanted to play with regex stuff.
    let re = Regex::new("([A-C]) ([X-Z])").unwrap();
    for line in utils::aoc_input_lines("day2.txt").unwrap() {
        let s = line.unwrap();
        let cap = re.captures(&s).unwrap();
        let opps = cap.get(1).unwrap().as_str().chars().next().unwrap();
        let aim = cap.get(2).unwrap().as_str().chars().next().unwrap();
        total += score2(opps, aim);
    }
    println!("Part2 Total score: {}", total);
}

pub fn run () {
    part1();
    part2();
}