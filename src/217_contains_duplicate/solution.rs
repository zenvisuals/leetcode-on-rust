use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // We use a hashset so we can get constant time access on lookup
        let mut s = HashSet::new();

        // If the number is not in the hashset, we add that number in
        // So that if it appears again, we treat that as a duplicate
        // As long as we find the first duplicate element
        // We end the function immediately since we already got our answer
        for n in nums {
            if (s.contains(&n)) {
                return true;
            } else {
                s.insert(n);
            }
        }
        false
    }
}