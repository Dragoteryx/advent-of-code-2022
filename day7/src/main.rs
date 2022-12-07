mod dir;
use dir::*;

const INPUT: &str = include_str!("real_input.txt");

fn directories(input: &str) -> Directory {
	let mut root = Directory::new();
  let mut path = Vec::new();

  for line in input.lines() {
    match line.split_whitespace().collect::<Vec<_>>().as_slice() {
      ["$", "cd", "/"] => path.clear(),
      ["$", "cd", ".."] => { path.pop(); }
      ["$", "cd", dir_name] => path.push(*dir_name),
      ["$", "ls"] => (),
      ["dir", dir_name] => root.dir_mut(&path[..])
        .unwrap_or_else(|| panic!("unknown directory `/{}`", path.join("/")))
        .add_dir(dir_name),
      [size, file_name] => root.dir_mut(&path[..])
        .unwrap_or_else(|| panic!("unknown directory `/{}`", path.join("/")))
        .add_file(file_name, size.parse().expect("invalid input")),
      _ => unreachable!()
    }
  }

  root
}

fn part1(root: &Directory) -> usize {
  root.recur()
    .map(|dir| dir.size())
    .filter(|size| *size < 100_000)
    .sum()
}

fn part2(root: &Directory) -> usize {
  let remaining = 70000000 - root.size();
  let required = 30000000 - remaining;

  root.recur()
    .map(|dir| dir.size())
    .filter(|size| *size >= required)
    .min()
    .unwrap()
}

fn main() {
  println!("--- day 7 ---");

  let root = directories(INPUT);
  println!("part 1 => {}", part1(&root));
  println!("part 2 => {}", part2(&root));
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("test_input.txt");

  #[cfg(test)]
  fn test_part1_part2() {
    let root = directories(INPUT);
    assert_eq!(part1(&root), 95437);
    assert_eq!(part2(&root), 24933642);
  }

}