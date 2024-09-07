fn main() {
    race(720, 850, 70);
}

fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }

    let catch_up = g as f64 * 3600.0 / (v2 as f64 - v1 as f64);

    let hours: i32 = (catch_up / 3600.0) as i32;
    let minutes: i32 = ((catch_up / 60.0) % 60.0) as i32;
    let seconds: i32 = (catch_up % 60.0) as i32;

    Some(vec![hours, minutes, seconds])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        // assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
        assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
        assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
        assert_eq!(race(820, 81, 550), None);
    }
}
