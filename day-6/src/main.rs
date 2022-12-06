use std::fs;

fn main() {
  let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
  let pt1_marker_length = 4;
  let pt2_marker_length = 14;

  let (pt1_marker_length, pt1_chars) = get_marker(pt1_marker_length, &input);
  let (pt2_marker_length, pt2_chars) = get_marker(pt2_marker_length, &input);

  println!("Part 1: {}. Chars: {}", pt1_marker_length, pt1_chars);
  println!("Part 2: {}. Chars: {}", pt2_marker_length, pt2_chars);
}

fn get_marker(marker_length: usize, input: &str) -> (usize, String) {
  for i in 0..input.len() {
    let four_chars = input.chars().skip(i).take(marker_length).collect::<String>();
    let mut unique_chars = String::new();
    for c in four_chars.chars() {
      if !unique_chars.contains(c) {
        unique_chars.push(c);
      }
    }
    if unique_chars.len() == marker_length {
      return (i + marker_length, four_chars);
    }
  }
  (0, "".to_string())
}
