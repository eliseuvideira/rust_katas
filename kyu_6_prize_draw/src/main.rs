fn main() {
    rank(
        "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin",
        vec![4, 2, 1, 4, 3, 1, 2],
        4,
    );
}

fn rank(first_name_list: &str, weights: Vec<i32>, position: usize) -> &str {
    if first_name_list == "" {
        // eew! better -> None / Some("..")
        return &"No participants";
    }

    let mut results: Vec<(&str, i32)> = first_name_list
        .split(',')
        .enumerate()
        .map(|(index, first_name)| {
            (
                first_name,
                calc_first_name(&first_name, *weights.get(index).unwrap()),
            )
        })
        .collect();

    if position > results.len() {
        return &"Not enough participants";
    }

    results.sort_by(|(a_name, a), (b_name, b)| if a != b { b.cmp(a) } else { a_name.cmp(b_name) });

    let result = results.get(position - 1).unwrap().0;

    result
}

fn calc_first_name(first_name: &str, weight: i32) -> i32 {
    let len = first_name.len() as i32;
    let sum: i32 = first_name
        .chars()
        .map(|ch| ((ch.to_ascii_lowercase() as u8) - b'a' + 1) as i32)
        .sum();

    let result = (len + sum) * weight;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(
            rank(
                "COLIN,AMANDBA,AMANDAB,CAROL,PauL,JOSEPH",
                vec![1, 4, 4, 5, 2, 1],
                4,
            ),
            "PauL"
        );
        assert_eq!(
            rank(
                "Addison,Jayden,Sofia,Michael,Andrew,Lily,Benjamin",
                vec![4, 2, 1, 4, 3, 1, 2],
                4,
            ),
            "Benjamin"
        );
        assert_eq!(
            rank(
                "Elijah,Chloe,Elizabeth,Matthew,Natalie,Jayden",
                vec![1, 3, 5, 5, 3, 6],
                2,
            ),
            "Matthew"
        );
        assert_eq!(
            rank(
                "Aubrey,Olivai,Abigail,Chloe,Andrew,Elizabeth",
                vec![3, 1, 4, 4, 3, 2],
                4,
            ),
            "Abigail"
        );
        assert_eq!(rank("Lagon,Lily", vec![1, 5], 2,), "Lagon");
    }
}
