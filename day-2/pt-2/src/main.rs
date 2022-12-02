use std::fs;

fn main() {
  let input = fs::read_to_string("input.txt").expect("Unable to read file");

  // Rock A / X
  // Paper B / Y
  // Scissors C / Z

  let mut score = 0;
  for line in input.lines() {
    let choices = line.split_whitespace().collect::<Vec<&str>>();
    let elf_choice = choices[0];
    let human_choice = choices[1];

    if human_choice == "Y" {
      score += get_choice_score(get_choice(get_draw_choice(elf_choice))) + 3;
    } else if human_choice == "Z" {
      score += get_choice_score(get_choice(get_win_choice(elf_choice))) + 6;
    } else if human_choice == "X" {
      score += get_choice_score(get_choice(get_lose_choice(elf_choice)));
    }
  }

  println!("Score: {}", score);
}

fn get_choice(choice: &str) -> &str {
  match choice {
    "A" | "X" => "ROCK",
    "B" | "Y" => "PAPER",
    "C" | "Z" => "SCISSORS",
    _ => panic!("Invalid choice"),
  }
}

fn get_choice_score(choice: &str) -> i32 {
  match choice {
    "ROCK" => 1,
    "PAPER" => 2,
    "SCISSORS" => 3,
    _ => panic!("Invalid score choice"),
  }
}

fn get_lose_choice(choice: &str) -> &str {
  match choice {
    "A" => "Z",
    "C" => "Y",
    "B" => "X",
    _ => panic!("Invalid lose choice"),
  }
}

fn get_win_choice(choice: &str) -> &str {
  match choice {
    "A" => "Y",
    "C" => "X",
    "B" => "Z",
    _ => panic!("Invalid win choice"),
  }
}

fn get_draw_choice(choice: &str) -> &str {
  match choice {
    "A" => "X",
    "C" => "Z",
    "B" => "Y",
    _ => panic!("Invalid draw choice"),
  }
}
