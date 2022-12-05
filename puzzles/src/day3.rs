use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use crate::utils;

fn score(rucksack: &str) -> i32 {
    let comp1 = &rucksack[0..rucksack.len()/2];
    let comp2 = &rucksack[rucksack.len()/2..rucksack.len()];
    let mut cache1 = HashSet::<char>::new();
    let mut cache2 = HashSet::<char>::new();
    let mut total = 0;


    /*
     * Cache the items from each compartment in a set.
     */
    for c in comp1.chars() {
        cache1.insert(c);
    }
    for c in comp2.chars() {
        cache2.insert(c);
    }

    /*
     * Score the matches in the intersection
     */
    for p in cache1.intersection(&cache2) {
        if ('a' as i32 <= *p as i32) && (*p as i32 <= 'z' as i32) {
            total += 1 + *p as i32 - 'a' as i32;
        } else if ('A' as i32 <= *p as i32) && (*p as i32 <= 'Z' as i32) {
            total += 27 + *p as i32 - 'A' as i32;
        }
    }
    //println!("{} {}: {}", comp1, comp2, total);

    return total
}

fn part1() {
    let mut total = 0;
    let re = Regex::new(r"([A-Za-z]+)").unwrap();
    for line in utils::aoc_input_lines("day3.txt").unwrap() {
        let s = line.unwrap();
        let cap = re.captures(&s).unwrap();
        let rucksack = cap.get(1).unwrap().as_str();
        total += score(rucksack);
    }
    println!("Part1 Total score: {}", total);
}

fn score_group(rucksacks: Vec<Result<String, std::io::Error>>) -> i32 {
    let mut caches = [HashSet::<char>::new(),
                                          HashSet::<char>::new(),
                                          HashSet::<char>::new()];
    let mut total = 0;

    /*
     * Cache the items from each compartment in a set.
     */
    for (i, r) in rucksacks.iter().enumerate() {
        let chs = r.as_deref();
        for c in chs.unwrap().chars() {
            caches[i].insert(c);
        }
    }
    /*
     * Reduce the first set, to those matching across all sets.
     */
    //caches[0].iter().filter (move |c| caches[1..].iter().all (|s| s.contains (c)));
    //let inter = caches[0].intersection(&caches[1]);
    let inter = caches
        .iter()
        .skip(1)
        .fold(caches[0].clone(), |acc, hs| {
                  acc.intersection(hs).cloned().collect()
              }); 


    /*
     * Score the matches in the intersection
     */
    for p in inter {
        if ('a' as i32 <= p as i32) && (p as i32 <= 'z' as i32) {
            total += 1 + p as i32 - 'a' as i32;
        } else if ('A' as i32 <= p as i32) && (p as i32 <= 'Z' as i32) {
            total += 27 + p as i32 - 'A' as i32;
        }
    }
    //println!("{:?}: {}", rucksacks, total);

    return total
}


fn part2() {
    let mut total = 0;
    for chunk in &utils::aoc_input_lines("day3.txt").unwrap().into_iter().chunks(3) {
        let rucksacks = chunk.collect_vec();
        total += score_group(rucksacks);
    }
    
    println!("Part2 Total score: {}", total);
}


pub fn run () {
    part1();
    part2();
}