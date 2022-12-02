use std::fs;

fn main() {
  let input = fs::read_to_string("input.txt").expect("Unable to read file");

  // Rock A / X
  // Paper B / Y
  // Scissors C / Z

  struct Scores {
    rock: i32,
    paper: i32,
    scissors: i32,
  }

  let scores = Scores {
    rock: 1,
    paper: 2,
    scissors: 3,
  };

  let mut score = 0;
  for line in input.lines() {
    let choices = line.split_whitespace().collect::<Vec<&str>>();
    let elf_choice = choices[0];
    let human_choice = choices[1];

    let choice_score = match human_choice {
      "X" => scores.rock,
      "Y" => scores.paper,
      "Z" => scores.scissors,
      _ => panic!("Invalid choice"),
    };

    if get_choice(elf_choice) == get_choice(human_choice) {
      score += choice_score + 3;
    } else if (elf_choice == "C" && human_choice == "X") || (elf_choice == "B" && human_choice == "Z") || (elf_choice == "A" && human_choice == "Y") {
      score += choice_score + 6;
    } else {
      score += choice_score;
    }
  }
  println!("Score: {}", score);
}

fn get_choice (choice: &str) -> &str {
  match choice {
    "A" | "X" => "Rock",
    "B" | "Y" => "Paper",
    "C" | "Z" => "Scissors",
    _ => panic!("Invalid choice"),
  }
}
