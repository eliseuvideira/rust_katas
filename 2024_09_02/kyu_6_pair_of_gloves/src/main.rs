use std::collections::HashSet;

fn main() {
    number_of_pairs(&["red", "red"]);
    ()
}

fn number_of_pairs(gloves: &[&str]) -> u32 {
    let mut total_pairs = 0;
    let mut seen: HashSet<&str> = HashSet::new();

    for glove in gloves {
        if seen.contains(glove) {
            total_pairs += 1;
            seen.remove(glove);
        } else {
            seen.insert(glove);
        }
    }

    total_pairs
}

#[cfg(test)]
mod tests {
    use super::number_of_pairs;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(arr: &[&str], expected: u32) {
        assert_eq!(
            number_of_pairs(arr),
            expected,
            "{ERR_MSG} with gloves = {arr:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(&["red", "red"], 1);
        dotest(&["red", "green", "blue"], 0);
        dotest(&["gray", "black", "purple", "purple", "gray", "black"], 3);
        dotest(&[], 0);
        dotest(
            &[
                "red", "green", "blue", "blue", "red", "green", "red", "red", "red",
            ],
            4,
        );
    }
}
