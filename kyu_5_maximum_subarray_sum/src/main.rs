fn main() {
    max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
}

fn max_sequence(seq: &[i32]) -> i32 {
    let mut current_sum = 0;
    let mut max_sum = 0;

    for item in seq.iter() {
        current_sum += item;

        if current_sum > max_sum {
            max_sum = current_sum;
        }
        if current_sum < 0 {
            current_sum = 0;
        }
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::max_sequence;

    #[test]
    fn sample_tests() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
    }
}
