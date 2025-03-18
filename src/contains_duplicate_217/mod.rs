use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    // We use a hashset so we can get constant time access on lookup
    let mut s = HashSet::new();

    // If the number is not in the hashset, we add that number in
    // So that if it appears again, we treat that as a duplicate
    // As long as we find the first duplicate element
    // We end the function immediately since we already got our answer
    for n in nums {
        if s.contains(&n) {
            return true;
        } else {
            s.insert(n);
        }
    }
    false
}

mod tests {
    use super::*;

    #[test]
    fn is_contains_duplicate() {
        let result: bool = contains_duplicate(vec![1, 2, 3, 3]);
        assert_eq!(result, true);
    }

    #[test]
    fn is_not_contains_duplicate() {
        let result: bool = contains_duplicate(vec![1, 2, 3, 4]);
        assert_eq!(result, false);
    }
}
