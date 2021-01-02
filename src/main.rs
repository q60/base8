fn main() {
  let alphabet: Vec<&str> = vec![
    "A", "B", "C", "D",
    "E", "F", "G", "H",
  ];
  let mut to_encode:  String = String::new();
  let mut bytes: Vec<String>;
  let mut odd:        String = String::new();
  std::io::stdin()
    .read_line(&mut to_encode)
    .expect("failed to read!!!");
  bytes = to_encode.trim()
    .as_bytes()
    .iter()
    .map(|byte| format!("{:0>8b}", byte))
    .collect::<Vec<String>>()
    .join("")
    .chars()
    .map(|char| char.to_string())
    .collect::<Vec<String>>();
  while bytes.join("").len() % 3 != 0 {
    bytes.push("00000000".to_string());
    odd.push('=');
  }
  for _ in 1..odd.len() {
    bytes.pop();
  }
  println!("{}{}", bytes.join("")
    .chars()
    .map(|char| char.to_string())
    .collect::<Vec<String>>()
    .chunks(3)                        // split bytes vec to triplets
    .map(|triplet|
      alphabet[usize::from_str_radix( //
        &triplet.join(""),            // using triplet as alphabet index
        2).unwrap()])                 //
    .collect::<Vec<&str>>()
    .join(""),
    odd
  );
}
