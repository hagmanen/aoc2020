
/*
Day 10, part 1: 2046
Day 10, part 2: 1157018619904
*/

pub fn day10() {
  let contents = std::fs::read_to_string("input10.txt").expect("Failed to read file");
  let adapters : std::collections::BTreeSet<u64> = contents.lines().map(|s| s.parse().expect(s)).collect();

  let mut one_steps : u64 = 0;
  let mut repeating : u64 = 0;
  let mut permuts : u64 = 1;
  let mut three_steps : u64 = 1; // last one is always 3
  let mut last : u64 = 0;
  for adapter in &adapters {
    match adapter - last {
      1 => {
        one_steps += 1;
        repeating += 1;},
      3 => {
        three_steps += 1;
        match repeating {
          0 => (),
          1 => (),
          2 => permuts *= 2,
          3 => permuts *= 4,
          4 => permuts *= 7,
          _ => panic!("fail"),
        }
        repeating = 0;
      },
      _ => panic!("invalid increase")
    }
    last = *adapter;
  }

  match repeating {
    0 => (),
    1 => (),
    2 => permuts *= 2,
    3 => permuts *= 4,
    4 => permuts *= 7,
    _ => panic!("fail"),
  }

  println!("Day 10, part 1: {}", one_steps * three_steps);
  println!("Day 10, part 2: {}", permuts);
}