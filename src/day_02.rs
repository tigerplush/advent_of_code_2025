const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

pub fn try_solve(input: &str) -> (usize, usize) {
    (part_one(input), part_two(input))
}

fn part_one(input: &str) -> usize {
    let mut invalid_ids = 0;
    let id_ranges: Vec<&str> = input.split(',').collect();
    for range in id_ranges {
        let numbers: Vec<&str> = range.split('-').collect();
        let low = numbers[0].parse::<u64>().unwrap();
        let high = numbers[1].parse::<u64>().unwrap();
        for id in low..=high {
            let string_id = id.to_string();
            let half_point = string_id.len() / 2;
            let lower = &string_id[0..half_point];
            let upper = &string_id[half_point..];
            if lower == upper {
                invalid_ids += id as usize;
            }
        }
    }
    invalid_ids
}

fn part_two(input: &str) -> usize {
    let mut invalid_ids = 0;
    let id_ranges: Vec<&str> = input.split(',').collect();
    for range in id_ranges {
        let numbers: Vec<&str> = range.split('-').collect();
        let low = numbers[0].parse::<u64>().unwrap();
        let high = numbers[1].parse::<u64>().unwrap();
        for id in low..=high {
            let string_id = id.to_string();
            let half_point = string_id.len() / 2;
            for chunk_size in 1..=half_point {
                let chunks = string_id
                    .as_bytes()
                    .chunks(chunk_size)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();

                let first = &chunks[0];
                if chunks.iter().all(|item| item == first) {
                    invalid_ids += id as usize;
                    break;
                }
            }
        }
    }
    invalid_ids
}

#[test]
fn day_02_solve_part_one() {
    assert_eq!(part_one(TEST_INPUT), 1227775554);
}

#[test]
fn day_02_solve_part_two() {
    assert_eq!(part_two(TEST_INPUT), 4174379265);
}
