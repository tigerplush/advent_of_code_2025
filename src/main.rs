use std::fs;

mod day_01;
mod day_02;
mod day_03;

fn main() {
    let input = fs::read_to_string("./input/day_03.txt").unwrap();
    let result = day_03::try_solve(&input);
    println!("{:?}", result);
}
