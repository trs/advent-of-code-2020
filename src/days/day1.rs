use crate::utils::request;

pub fn run() {
  let input = request::get("https://adventofcode.com/2020/day/1/input").unwrap();

  let entries: Vec<i32> = input.lines().map(|entry| entry.parse::<i32>().unwrap()).collect();

  let part_1 = find_two(&entries).unwrap();
  println!("Part 1: {}", part_1);

  let part_2 = find_three(&entries).unwrap();
  println!("Part 2: {}", part_2);
}

fn find_two(entries: &Vec<i32>) -> Option<i32> {
  let length = entries.len() - 1;
  let mut i = 0;

  loop {
    if i > length {
      return None;
    }

    let compare = entries[i];

    for j in i+1..length {
      let value = entries[j];
      if compare + value == 2020 {
        return Some(compare * value);
      }
    };

    i = i + 1;
  }
}

fn find_three(entries: &Vec<i32>) -> Option<i32> {
  let length = entries.len() - 1;
  let mut i = 0;

  loop {
    if i > length {
      return None;
    }

    let comp_1 = entries[i];

    for j in i+1..length {
      let comp_2 = entries[j];

      for k in j+1..length {
        let comp_3 = entries[k];

        if comp_1 + comp_2 + comp_3 == 2020 {
          return Some(comp_1 * comp_2 * comp_3);
        }
      }
    };

    i = i + 1;
  }
}
