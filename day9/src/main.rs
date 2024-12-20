use std::fs;

fn part1(input: &[u32]) -> u64 {
    let mut left_seek = 0;
    let mut right_seek = input.len() - 1;
    debug_assert_eq!(right_seek % 2, 0);

    let mut block_position = 0;
    let mut num_unmoved = input[right_seek];
    let mut checksum = 0u64;

    while left_seek < right_seek {
        // Checksum for unmoved file
        let unmoved_file_size = input[left_seek];
        for offset in 0..unmoved_file_size {
            let file_id = left_seek / 2;
            checksum += (block_position + offset) as u64 * file_id as u64;
        }
        block_position += unmoved_file_size;

        // Checksum for moved file
        let free_space_size = input[left_seek + 1];
        for offset in 0..free_space_size {
            if num_unmoved == 0 {
                right_seek -= 2;
                num_unmoved = input[right_seek];
            }
            let file_id = right_seek / 2;
            checksum += (block_position + offset) as u64 * file_id as u64;
            num_unmoved -= 1;
        }
        block_position += free_space_size;
        left_seek += 2;
    }

    for offset in 0..num_unmoved {
        let file_id = right_seek / 2;
        checksum += (block_position + offset) as u64 * file_id as u64;
    }

    checksum
}

fn part2(_input: &[u32]) -> u64 {
    todo!()
}

fn main() {
    let input = fs::read_to_string("day9/data/input.txt")
        .unwrap()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
