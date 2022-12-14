use std::fs;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let file_input = fs::read_to_string("input.txt")
        .expect("Failed to read the input file.");
    let groups = file_input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|group| {
            group.split('\n')
                .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
                .collect::<Vec<i32>>()
        });

    //part 1
    let mut totals = groups.map(|group| group.iter().sum::<i32>());
    let max_sum = totals.max();

    println!("The larget sum of integers in the input file is: {:?}", max_sum);

    //part 2
    totals.sort();

}
