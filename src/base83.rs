pub fn encode(value: u32, length: u32) -> String {
  let characters: Vec<char> = String::from(
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#$%*+,-.:;=?@[]^_{|}~",
  )
  .chars()
  .collect();
  let mut result = String::new();

  for i in 1..length + 1 {
    let digit: u32 = (value / u32::pow(83, length - i)) % 83;
    result.push_str(&characters[digit as usize].to_string());
  }

  result
}

pub fn decode(str: &str) -> usize {
  let characters: Vec<char> = String::from(
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#$%*+,-.:;=?@[]^_{|}~",
  )
  .chars()
  .collect();
  let mut value = 0;

  let str: Vec<char> = str.chars().collect();

  for i in 0..str.len() {
    let digit: usize = characters.iter().position(|&r| r == str[i]).unwrap();
    value = value * 83 + digit;
  }

  value
}

#[cfg(test)]
mod test {
  use super::{decode, encode};

  #[test]
  fn encode83() {
    let v = decode("~$");
    assert_eq!(v, 6869);
  }

  #[test]
  fn decode83() {
    let str = encode(6869, 2);
    assert_eq!(str, "~$");
  }
}
