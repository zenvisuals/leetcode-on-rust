use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // Initiate a hashmap to count the frequency of the numbers
    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    let nums_length = nums.len();

    for n in nums {
        *freq_map.entry(n).or_insert(0) += 1;
    }

    // Initiate a vector where the index is the frequency, max length is the length of the numbers
    let mut freq_vec: Vec<Vec<i32>> = vec![Vec::new(); nums_length + 1];

    for (k, v) in freq_map {
        freq_vec[v as usize].push(k);
    }

    let mut result: Vec<i32> = Vec::new();

    // Initiate the current index to point to the right most element of the frequency vector
    let mut current_index = nums_length;

    // We will move backward on the frequency vector to collect the top K frequent elements
    // Until result's length reach k, we will keep adding the element
    while result.len() != k as usize {
        if freq_vec[current_index].len() > 0 {
            result.push(freq_vec[current_index].pop().unwrap())
        } else {
            current_index -= 1;
        }
    }

    return result;
}

mod tests {
    use super::*;

    #[test]
    fn pick_top_two_elements() {
        let result: Vec<i32> = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn pick_top_one_element() {
        let result: Vec<i32> = top_k_frequent(vec![1], 1);
        assert_eq!(result, vec![1]);
    }
}