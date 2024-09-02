use std::collections::HashSet;

fn main() -> () {
    find_deleted_number(&[1, 2, 3], &[3, 2]);
    ()
}

fn find_deleted_number(list: &[u16], mixed_list: &[u16]) -> Option<u16> {
    let mut hashset: HashSet<u16> = HashSet::new();

    for item in mixed_list {
        hashset.insert(item.clone());
    }

    for item in list {
        if !hashset.contains(&item) {
            return Some(item.clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::find_deleted_number;

    #[test]
    fn basic() {
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 6, 7, 8, 1, 9]),
            Some(5)
        );
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 6, 7, 8, 9, 5]),
            Some(1)
        );
        assert_eq!(
            find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 1, 7, 8, 9, 5]),
            Some(6)
        );
    }
}
