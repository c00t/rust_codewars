#[cfg(test)]
mod tests {
    use super::*;    
    #[test]
    fn test_abcde() {
        assert_eq!(count_duplicates("abcde"), 0);
    }
    
    #[test]
    fn test_abcdea() {
        assert_eq!(count_duplicates("abcdea"), 1);
    }
    
    #[test]
    fn test_indivisibility() {
        assert_eq!(count_duplicates("indivisibility"), 1);
    }

    #[test]
    fn test_abcdeaB() {
        assert_eq!(count_duplicates("abcdeaB"), 1);
    }

}
use std::{collections::hash_map::HashMap};
// distinct case-insensitive chars duplicates
// more functional-style solution
// <fold> usage
// <entry> usage
// <or_insert> usage
fn count_duplicates_func(text: &str) -> u32 {
    return text
        .to_lowercase()
        .chars()
        .fold(HashMap::<char, usize>::new(), |mut m, c| {
            *(m.entry(c).or_insert(0)) += 1;
            m
        })
        .values()
        .filter(|n| n >= &&2)
        .count() as u32;
}
// distinct case-insensitive chars duplicates
fn count_duplicates(text: &str) -> u32 {
    // Your code goes here
    let text = text.to_lowercase();
    let mut char_hashmap = HashMap::new();

    for ch in text.chars() {
        if char_hashmap.contains_key(&ch) {
            if let Some(x) = char_hashmap.get_mut(&ch) {
                *x += 1;
            }
        }else{
            char_hashmap.insert(ch, 1);
        }
    }
    char_hashmap.iter().filter(|&(key,val)| *val != 1).count() as u32
}