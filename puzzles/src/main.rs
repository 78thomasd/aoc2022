use std::time::Instant;
mod day1;

fn main() {
    let mut days = Vec::new();
    let mut idx = 1;
    days.push(day1::run_all);

    for day in days {
        println!(">>> START:   Day {}:", idx);
        let start = Instant::now();
        let _err = day();
        let duration = start.elapsed();
        println!("<<< END:     Day {}: time {:?}", idx, duration);
        idx += 1;
    }
}
