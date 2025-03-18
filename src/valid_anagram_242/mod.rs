use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    // Setup a hashmap where the key is the character and value is the number of occurence
    let mut s_map: HashMap<char, i32> = HashMap::new();
    let mut t_map: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        *s_map.entry(c).or_insert(0) += 1;
    }
    for c in t.chars() {
        *t_map.entry(c).or_insert(0) += 1
    }

    // The two strings are considered valid anagram if they contain the same characters with the same amount of occurence
    s_map == t_map
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_anagram() {
        let result = is_anagram(String::from("racecar"), String::from("carrace"));
        assert_eq!(result, true);
    }

    #[test]
    fn invalid_anagram() {
        let result: bool = is_anagram(String::from("jar"), String::from("jam"));
        assert_eq!(result, false);
    }
}