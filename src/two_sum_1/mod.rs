use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Create a hashmap with the number as the key and index as the value
    let mut m: HashMap<i32, i32> = HashMap::new();
    let mut result = Vec::new();
    for i in 0..nums.len() {
        let index: i32 = i as i32;
        let diff: i32 = target - nums[i];

        if m.contains_key(&diff) && m[&diff] != index {
            result.push(m[&diff]);
            result.push(index);
            break
        }
        // Before we insert new entry into the hashmap, we do the above check first
        // This handles the edge case where you get [3, 3] and it expects the result [0, 1]
        // If we put the insertion before the check, it will overwrite the existing key with the new one, becoming [1, 1] as the result
        m.insert(nums[i], index);
    }
    return result;
}

mod tests {
    use super::*;

    #[test]
    fn first_two_elements() {
        let result: Vec<i32> = two_sum(vec![3, 4, 5, 6], 7);
        assert_eq!(result, vec![0, 1])
    }

    #[test]
    fn first_and_last_elements() {
        let result: Vec<i32> = two_sum(vec![4, 5, 6], 10);
        assert_eq!(result, vec![0, 2])
    }

    #[test]
    fn duplicate_numbers() {
        let result: Vec<i32> = two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1])
    }
}
