use crate::utils::request;

pub fn run() {
  let input = request::get("https://adventofcode.com/2020/day/1/input").unwrap();

  let entries: Vec<i32> = input.lines().map(|entry| entry.parse::<i32>().unwrap()).collect();

  let value = find(&entries).unwrap();
  println!("{}", value);
}

fn find(entries: &Vec<i32>) -> Option<i32> {
  let length = entries.len() - 1;
  let mut i = 0;

  loop {
    if i > length {
      return None;
    }

    let compare = entries[i];

    for j in i..length {
      if j > length {
        break;
      }

      if j != i {
        let value = entries[j];
        if compare + value == 2020 {
          return Some(compare * value);
        }
      }
    };

    i = i + 1;
  }
}
