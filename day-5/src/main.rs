use std::fs;

// /$$   /$$ /$$$$$$ /$$       /$$              /$$$$$$  /$$       /$$             /$$$$$$$$ /$$    /$$    /$$ /$$$$$$$$  /$$$$$$
// | $$  /$$/|_  $$_/| $$      | $$             /$$__  $$| $$      | $$            | $$_____/| $$   | $$   | $$| $$_____/ /$$__  $$
// | $$ /$$/   | $$  | $$      | $$            | $$  \ $$| $$      | $$            | $$      | $$   | $$   | $$| $$      | $$  \__/
// | $$$$$/    | $$  | $$      | $$            | $$$$$$$$| $$      | $$            | $$$$$   | $$   |  $$ / $$/| $$$$$   |  $$$$$$
// | $$  $$    | $$  | $$      | $$            | $$__  $$| $$      | $$            | $$__/   | $$    \  $$ $$/ | $$__/    \____  $$
// | $$\  $$   | $$  | $$      | $$            | $$  | $$| $$      | $$            | $$      | $$     \  $$$/  | $$       /$$  \ $$
// | $$ \  $$ /$$$$$$| $$$$$$$$| $$$$$$$$      | $$  | $$| $$$$$$$$| $$$$$$$$      | $$$$$$$$| $$$$$$$$\  $/   | $$$$$$$$|  $$$$$$/
// |__/  \__/|______/|________/|________/      |__/  |__/|________/|________/      |________/|________/ \_/    |________/ \______/ 

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rows = parts[0].split("\n").collect::<Vec<&str>>();
    let instructions = parts[1].lines();
    let mut grid: Vec<Vec<String>> = vec![];

    for row in rows {
      let mut split_row = row
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

      let empty_string = String::from("    ");
      if split_row.len() < 9 {
        let length = split_row.len();
        split_row[length - 1] = split_row[length - 1].clone() + " ";
        split_row.resize(9, empty_string);
      }

      grid.push(split_row);
    };

    let mut grid_pt_2 = grid.clone();

    grid.reverse();
    grid_pt_2.reverse();

    // // Part One
    for i in instructions.clone() {
      let instr: Vec<&str> = i.split(" ").collect();
      let count: usize = instr[1].parse().unwrap();
      let from: usize = instr[3].parse().unwrap();
      let to: usize = instr[5].parse().unwrap();

      for _ in 0..count {
        let empty_to = get_empty_col_index(grid.clone(), to);
        let empty_from = get_empty_col_index(grid.clone(), from);
        if empty_to == 0 {
          grid.push(vec![String::from("    "); 9]);
        }
        if empty_from == 0 {
          grid.push(vec![String::from("    "); 9]);
        }
        let empty_from = get_empty_col_index(grid.clone(), from) - 1;
        let empty_to = get_empty_col_index(grid.clone(), to);
        let removed = grid[empty_from][from - 1].clone();
        grid[empty_from][from - 1] = String::from("    ");
        grid[empty_to][to - 1] = removed;
      }
    }

    let top_boxes = get_top_boxes(grid.clone());
    println!("PT-1 => {:?}", top_boxes.join(""));

    // Part Two
    for i in instructions.clone() {
      let instr: Vec<&str> = i.split(" ").collect();
      let count: usize = instr[1].parse().unwrap();
      let from: usize = instr[3].parse().unwrap();
      let to: usize = instr[5].parse().unwrap();

      let mut items_to_move: Vec<String> = vec![];
      for _ in 0..count {
        let empty_from = get_empty_col_index(grid_pt_2.clone(), from);
        if empty_from == 0 {
          grid_pt_2.push(vec![String::from("    "); 9]);
        }
        let empty_from = get_empty_col_index(grid_pt_2.clone(), from) - 1;
        let removed = grid_pt_2[empty_from][from - 1].clone();
        grid_pt_2[empty_from][from - 1] = String::from("    ");
        items_to_move.push(removed);
      }

      items_to_move.reverse();

      for i in 0..count {
        let empty_to = get_empty_col_index(grid_pt_2.clone(), to);
        if empty_to == 0 {
          grid_pt_2.push(vec![String::from("    "); 9]);
        }
        let empty_to = get_empty_col_index(grid_pt_2.clone(), to);
        grid_pt_2[empty_to][to - 1] = items_to_move[i].clone();
      }
    }

    let top_boxes = get_top_boxes(grid_pt_2.clone());
    println!("PT-2 => {:?}", top_boxes.join(""));
}

fn get_empty_col_index(grid: Vec<Vec<String>>, col: usize) -> usize {
  for (i, row) in grid.iter().enumerate() {
    if row[col - 1] == "    " {
      return i;
    }
  }
  0
}

fn get_top_boxes(grid: Vec<Vec<String>>) -> Vec<String> {
  let mut top_boxes: Vec<String> = vec![];
  for i in 0..9 {
    let last = get_empty_col_index(grid.clone(), i + 1);
    let b = grid[last - 1][i].clone().replace("[", "").replace("]", "").trim().to_string();
    top_boxes.push(b);
  }
  top_boxes
}
