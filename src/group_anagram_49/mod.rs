use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut m: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for s in strs {
        // The problem statement mentions the input will only contain lowercase alphabets [a-z]
        // So we can use an array of a fixed size to contain the occurences of the characters
        // Two words are anagram of each other if they contain the same occurences of characters
        // We use u8 to reduce memory footprint and runtime
        // But this assumes a character won't appear more than 255 times
        let mut occurence: [u8; 26] = [0; 26];
        for c in s.chars() {
            let c_index: u8 = c.to_digit(36).unwrap() as u8 - 'a'.to_digit(36).unwrap() as u8;
            occurence[c_index as usize] += 1;
        }
        // If we detect another anagram, we simply add that word in the list
        m.entry(occurence).or_insert(Vec::new()).push(s);
    }

    // We return the list of words grouped by their anagrams
    m.into_values().collect::<Vec<Vec<String>>>()
}

mod tests {
    use super::*;

    #[test]
    fn words_grouped_by_anagram() {
        let mut result: Vec<Vec<String>> = group_anagrams(vec![
            String::from("act"),
            String::from("pots"),
            String::from("tops"),
            String::from("cat"),
            String::from("stop"),
            String::from("hat")
        ]);
        result.sort();
        let mut expected: Vec<Vec<String>> = vec![
            vec![String::from("pots"), String::from("tops"), String::from("stop")],
            vec![String::from("hat")],
            vec![String::from("act"), String::from("cat")],
        ];
        expected.sort();
        assert_eq!(result, expected)
    }

    #[test]
    fn single_word() {
        let result: Vec<Vec<String>> = group_anagrams(vec![String::from("x")]);
        assert_eq!(result, vec![vec![String::from("x")]])
    }

    #[test]
    fn empty_word() {
        let result: Vec<Vec<String>> = group_anagrams(vec![String::from("")]);
        assert_eq!(result, vec![vec![String::from("")]])
    }
}
