fn main() {
    solve("Code");
    ()
}

fn solve(s: &str) -> String {
    let len = s.len();
    let mut lower_count = 0;
    let mut upper_count = 0;
    for l in s.chars() {
        if l.is_lowercase() {
            lower_count += 1;
        } else {
            upper_count += 1;
        }

        if lower_count > (len / 2) {
            return s.to_lowercase();
        }
        if upper_count > (len / 2) {
            return s.to_uppercase();
        }
    }

    s.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solve("code"), "code");
        assert_eq!(solve("CODe"), "CODE");
        assert_eq!(solve("COde"), "code");
        assert_eq!(solve("Code"), "code");
    }
}
