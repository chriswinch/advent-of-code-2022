use std::fs;
use std::collections::HashSet;

fn main() {
  let input = fs::read_to_string("input.txt").expect("Unable to read file");
  let mut sum = 0;

  for rucksack in input.lines() {
    let (left, right) = rucksack.split_at(rucksack.len() / 2);

    let compartment_one: HashSet<char> = left.chars().collect();
    let compartment_two: HashSet<char> = right.chars().collect();

    let common_letters: HashSet<char> = compartment_one.intersection(&compartment_two).copied().collect();

    for c in common_letters {
      sum += get_letter_value(c);
    }
  }
  println!("Sum: {}", sum);

  // part two
  let mut badge_sum = 0;
  let groups: Vec<&str> = input.lines().collect::<Vec<_>>();
  let chunks = groups.chunks(3);
  for chunk in chunks {
    let set1 = chunk[0].chars().collect::<HashSet<char>>();
    let set2 = chunk[1].chars().collect::<HashSet<char>>();
    let set3 = chunk[2].chars().collect::<HashSet<char>>();

    let common_chars: HashSet<char> = set1.intersection(&set2).cloned().collect();
    let common_chars = common_chars.intersection(&set3);
    for c in common_chars {
      badge_sum += get_letter_value(*c);
    }
  }
  println!("Badge Sum: {}", badge_sum);
}

fn get_letter_value(c: char) -> u32 {
  if c.is_lowercase() {
    let initial_char_code = "a".chars().next().unwrap() as u32;
    return (c as u32).wrapping_sub(initial_char_code) + 1;
  } else {
    let initial_char_code = "A".chars().next().unwrap() as u32;
    return (c as u32).wrapping_sub(initial_char_code) + 27;
  }
}
