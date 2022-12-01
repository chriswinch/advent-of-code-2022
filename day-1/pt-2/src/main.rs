use std::fs;

fn main() {
  let contents = fs::read_to_string("../input.txt").expect("can't find file");

  let split: Vec<Vec<u64>> = contents.split("\n\n")
    .map(|par| par.lines().map(|l| l.parse().unwrap()).collect())
    .collect();

  let mut summed_vals: Vec<u64> = vec![];
  for line in split {
    let sum: u64 = line.iter().sum();
    summed_vals.push(sum);
  }

  summed_vals.sort();
  println!("total: {}", summed_vals.iter().rev().take(3).sum::<u64>());
}
