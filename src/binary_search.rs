pub fn binary_search(haystack: &[i32], needle: i32) -> bool {
    let mut low = 0;
    let mut high = haystack.len();

    while low < high {
        let mid = (low + high) / 2;
        let mid_element = haystack[mid];

        match needle.cmp(&mid_element) {
            std::cmp::Ordering::Less => high = mid,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => low = mid + 1,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let foo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420];

        assert!(binary_search(&foo, 69));
        assert!(!binary_search(&foo, 1336));
        assert!(binary_search(&foo, 69420));
        assert!(!binary_search(&foo, 69421));
        assert!(binary_search(&foo, 1));
        assert!(!binary_search(&foo, 0));
    }
}
