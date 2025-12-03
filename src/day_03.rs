const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

pub fn try_solve(input: &str) -> (usize, usize) {
    (part_one(input), part_two(input))
}

fn part_one(input: &str) -> usize {
    let mut joltage = 0;
    for line in input.lines() {
        joltage += part_one_parse_line(line);
    }
    joltage
}

fn part_two(input: &str) -> usize {
    let mut joltage = 0;
    for line in input.lines() {
        joltage += part_two_parse_line(line);
    }
    joltage
}

fn part_one_parse_line(line: &str) -> usize {
    let mut largest = 0;
    for (index, first) in line.chars().enumerate() {
        for second in line.chars().skip(index + 1) {
            let concatenated = first.to_string() + &second.to_string();
            let joltage = concatenated.parse::<usize>().unwrap();
            if joltage > largest {
                largest = joltage
            }
        }
    }
    largest
}

fn part_two_parse_line(line: &str) -> usize {
    let mut current_index = 0;
    let mut batteries = Vec::new();
    for index in 0..11 {
        let mut samples = line[current_index..(line.len() - 11 + index)]
            .chars()
            .enumerate()
            .map(|(index, char)| (index, char))
            .collect::<Vec<(usize, char)>>();
        samples.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));
        let (first_index, first_max) = samples.first().unwrap();
        batteries.push(*first_max);
        current_index += first_index + 1;
    }

    let second_max = &line[current_index..].chars().max_by_key(|f| *f).unwrap();
    batteries.push(*second_max);
    let joltage_string: String = batteries.iter().collect();
    joltage_string.parse::<usize>().unwrap()
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(TEST_INPUT), 357);
}

#[test]
fn test_parse_line() {
    assert_eq!(
        part_one_parse_line(TEST_INPUT.lines().skip(0).next().unwrap()),
        98
    );
    assert_eq!(
        part_one_parse_line(TEST_INPUT.lines().skip(1).next().unwrap()),
        89
    );
    assert_eq!(
        part_one_parse_line(TEST_INPUT.lines().skip(2).next().unwrap()),
        78
    );
    assert_eq!(
        part_one_parse_line(TEST_INPUT.lines().skip(3).next().unwrap()),
        92
    );
}

#[test]
fn test_parse_line_part_two() {
    assert_eq!(
        part_two_parse_line(TEST_INPUT.lines().skip(0).next().unwrap()),
        987654321111
    );
    assert_eq!(
        part_two_parse_line(TEST_INPUT.lines().skip(1).next().unwrap()),
        811111111119
    );
    assert_eq!(
        part_two_parse_line(TEST_INPUT.lines().skip(2).next().unwrap()),
        434234234278
    );
    assert_eq!(
        part_two_parse_line(TEST_INPUT.lines().skip(3).next().unwrap()),
        888911112111
    );
}
