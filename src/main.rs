use std::fs;

mod day_01;

fn main() {
    let input = fs::read_to_string("./input/day_01.txt").unwrap();
    let result = day_01::try_solve(&input);
    println!("{:?}", result);
}

