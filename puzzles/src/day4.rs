use regex::Regex;
use crate::utils;

fn score(r1: (&str, &str), r2: (&str, &str)) -> i32 {
    let a = (r1.0.parse::<i32>().unwrap(), r1.1.parse::<i32>().unwrap());
    let b = (r2.0.parse::<i32>().unwrap(), r2.1.parse::<i32>().unwrap());

    //println!("{:?} {:?}", a, b);
    if a.0 >= b.0 && a.1 <= b.1 {
        return 1;
    } else if b.0 >= a.0 && b.1 <= a.1 {
        return 1;
    }
    return 0;
}

fn part1() {
    let mut total = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for line in utils::aoc_input_lines("day4.txt").unwrap() {
        let s = line.unwrap();
        let cap = re.captures(&s).unwrap();
        let r1 = (cap.get(1).unwrap().as_str(),
                  cap.get(2).unwrap().as_str());
        let r2 = (cap.get(3).unwrap().as_str(),
                  cap.get(4).unwrap().as_str());
        total += score(r1, r2);
    }
    println!("Part1 Total score: {}", total);
}

fn overlaps(r1: (&str, &str), r2: (&str, &str)) -> i32 {
    let a = (r1.0.parse::<i32>().unwrap(), r1.1.parse::<i32>().unwrap());
    let b = (r2.0.parse::<i32>().unwrap(), r2.1.parse::<i32>().unwrap());

    //println!("{:?} {:?}", a, b);
    if a.0 >= b.0 && a.0 <= b.1 {
        return 1;
    } else if a.1 >= b.0 && a.1 <= b.1 {
        return 1;
    }
    return score(r1, r2);
}

fn part2() {
    let mut total = 0;
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for line in utils::aoc_input_lines("day4.txt").unwrap() {
        let s = line.unwrap();
        let cap = re.captures(&s).unwrap();
        let r1 = (cap.get(1).unwrap().as_str(),
                  cap.get(2).unwrap().as_str());
        let r2 = (cap.get(3).unwrap().as_str(),
                  cap.get(4).unwrap().as_str());
        total += overlaps(r1, r2);
    }
    println!("Part2 Total score: {}", total);
}


pub fn run () {
    part1();
    part2();
}