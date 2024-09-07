fn main() {
    remove_parentheses("Hello (World!)");
}

fn remove_parentheses(s: &str) -> String {
    let mut result = String::new();
    let mut level = 0;

    for c in s.chars() {
        match c {
            '(' => level += 1,
            ')' => level -= 1,
            c => {
                if level == 0 {
                    result.push(c);
                }
            }
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::remove_parentheses;

    #[test]
    fn sample_tests() {
        assert_eq!(
            remove_parentheses("example(unwanted thing)example"),
            "exampleexample"
        );
        assert_eq!(
            remove_parentheses("example (unwanted thing) example"),
            "example  example"
        );
        assert_eq!(remove_parentheses("a (bc d)e"), "a e");
        assert_eq!(remove_parentheses("a(b(c))"), "a");
        assert_eq!(
            remove_parentheses("hello example (words(more words) here) something"),
            "hello example  something"
        );
        assert_eq!(
            remove_parentheses("(first group) (second group) (third group)"),
            "  "
        );
    }
}
