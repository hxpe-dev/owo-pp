pub fn owoify(input: &str) -> String {
  let mut result = String::new();

  for c in input.chars() {
      match c {
          'r' | 'l' => result.push('w'),  // Change 'r' and 'l' to 'w'
          'R' | 'L' => result.push('W'),  // Change 'R' and 'L' to 'W'
          _ => result.push(c),           // Otherwise, keep the character as is
      }
  }

  // Add "owo" at randomly
  if rand::random::<f64>() > 0.5 {
      result.push_str(" owo");
  }

  result
}