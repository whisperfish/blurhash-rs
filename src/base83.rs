static CHARACTORS: [char; 83] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',
    'v', 'w', 'x', 'y', 'z', '#', '$', '%', '*', '+', ',', '-', '.', ':', ';', '=', '?', '@', '[',
    ']', '^', '_', '{', '|', '}', '~',
];

pub fn encode(value: u32, length: u32) -> String {
    let mut result = String::new();

    for i in 1..=length {
        let digit: u32 = (value / u32::pow(83, length - i)) % 83;
        result.push_str(&CHARACTORS[digit as usize].to_string());
    }

    result
}

pub fn decode(str: &str) -> usize {
    let mut value = 0;

    let str: Vec<char> = str.chars().collect();

    for i in 0..str.len() {
        let digit: usize = CHARACTORS.iter().position(|&r| r == str[i]).unwrap();
        value = value * 83 + digit;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn encode83() {
        let str = encode(6869, 2);
        assert_eq!(str, "~$");
    }

    #[test]
    fn decode83() {
        let v = decode("~$");
        assert_eq!(v, 6869);
    }
}
