use std::fs;

fn main() {
  let input = fs::read_to_string("input.txt").expect("Unable to read file");

  // part one
  let mut pt_one_sum = 0;
  for line in input.lines() {
    let split_str: Vec<&str> = line.split(",").collect();
    let first = split_str[0];
    let last = split_str[1];

    let split_first: Vec<i32> = first.split("-").map(|s| s.parse().unwrap()).collect();
    let split_last: Vec<i32> = last.split("-").map(|s| s.parse().unwrap()).collect();

    let first_start = split_first[0];
    let first_end = split_first[1];
    let last_start = split_last[0];
    let last_end = split_last[1];

    if first_start >= last_start && first_end <= last_end ||
        last_start >= first_start && last_end <= first_end {
      pt_one_sum += 1;
    }
  }
  println!("Part One: {}", pt_one_sum);

  // part two
  let mut pt_two_sum = 0;
  for line in input.lines() {
    let split_str: Vec<&str> = line.split(",").collect();
    let first = split_str[0];
    let last = split_str[1];

    let split_first: Vec<i32> = first.split("-").map(|s| s.parse().unwrap()).collect();
    let split_last: Vec<i32> = last.split("-").map(|s| s.parse().unwrap()).collect();

    let first_start = split_first[0];
    let first_end = split_first[1];
    let last_start = split_last[0];
    let last_end = split_last[1];

    if first_start <= last_end && last_start <= first_end {
      pt_two_sum += 1;
    }
  }
  println!("Part Two: {}", pt_two_sum);

}
