use std::fs;

mod day_01;
mod day_02;

fn main() {
    let input = fs::read_to_string("./input/day_02.txt").unwrap();
    let result = day_02::try_solve(&input);
    println!("{:?}", result);
}

