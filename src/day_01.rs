const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

pub fn try_solve(input: &str) -> (usize, usize) {
    (part_one(input), part_two(input))
}

fn part_one(input: &str) -> usize {
    let mut dial = 50;
    let mut zeroes = 0;
    for line in input.lines() {
        let direction = line.chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();
        match direction {
            'L' => dial -= value,
            'R' => dial += value,
            _ => (),
        }

        dial = wrap(dial, 0, 99);

        if dial == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

fn part_two(input: &str) -> usize {
    let mut dial = 50;
    let mut zeroes = 0;
    for line in input.lines() {
        let direction = line.chars().nth(0).unwrap();
        let value = line[1..].parse::<i32>().unwrap();
        // contains the new, unwrapped dial value
        let new_dial = match direction {
            'L' => dial - value,
            'R' => dial + value,
            _ => panic!("Could not parse {}", line),
        };
        zeroes += (new_dial / 100).abs() as usize;
        if dial != 0 && new_dial <= 0 {
            zeroes += 1;
        }
        dial = wrap(new_dial, 0, 99);
    }
    zeroes
}

fn wrap(value: i32, min: i32, max: i32) -> i32 {
    min + (value - min).rem_euclid(max - min + 1)
}

#[test]
fn day_01_solve_part_one() {
    assert_eq!(part_one(TEST_INPUT), 3);
}

#[test]
fn day_01_solve_part_two() {
    assert_eq!(part_two(TEST_INPUT), 6);
    assert_eq!(part_two("R1000"), 10);
}

#[test]
fn test_wrap() {
    assert_eq!(wrap(50_i32 + 21, 0, 99), 71);
    assert_eq!(wrap(50_i32 - 51, 0, 99), 99);
    assert_eq!(wrap(50_i32 + 51, 0, 99), 1);
    assert_eq!(wrap(50_i32 - 200, 0, 99), 50);
}
