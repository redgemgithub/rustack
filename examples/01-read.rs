fn main() {
  for line in std::io::stdin().lines() {
    let mut stack = vec![];
    if let Ok(line) = line {
      for word in line.split(" ").collect::<Vec<_>>() {
        if let Ok(parsed) = word.parse::<i32>() {
          stack.push(parsed);
        } else {
          match word {
            "+" => add(&mut stack),
            _ => panic!("{word:?} could not be parsed"),
          }
        }
      }
      println!("stack: {stack:?}");
    }
  }
}

fn add(stack: &mut Vec<i32>) {
  let lhs = stack.pop().unwrap();
  let rhs = stack.pop().unwrap();
  stack.push(lhs + rhs);
}
