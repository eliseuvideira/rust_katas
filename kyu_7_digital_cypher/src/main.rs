const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    encode("scout".to_string(), 1939);
}

fn encode(msg: String, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let keys: Vec<i32> = n
        .to_string()
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();
    let max_len = keys.len();

    let mut current_len = 0;
    for letter in msg.chars() {
        if let Some(index) = ALPHABET.find(letter) {
            let current_number = index + 1;
            if let Some(key_value) = keys.get(current_len % max_len) {
                result.push(current_number as i32 + key_value);
            }
        }
        current_len += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_tests() {
        assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
        assert_eq!(
            encode("masterpiece".to_string(), 1939),
            vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]
        );
    }
}
