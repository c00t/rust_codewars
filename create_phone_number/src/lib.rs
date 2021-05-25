//! Write a function that accepts an array of 10 integers 
//! (between 0 and 9), that returns a string of those numbers 
//! in the form of a phone number.

use std::fmt::format;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_correct() {
        assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]), "(123) 456-7890");
        assert_eq!(create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), "(111) 111-1111");
        assert_eq!(create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]), "(123) 456-7899");
    }
}
// # Same solution, but more concise
// let s: String = numbers.into_iter().map(|i| i.to_string()).collect();  
// //use slice to fill {}
// format!("({}) {}-{}", &s[..3], &s[3..6], &s[6..])
// # and a more ... version
// format!("({}{}{}) {}{}{}-{}{}{}{}", x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7], x[8], x[9])
// input &[1,2,3,4,5,6,7,8,9,0] => (123) 456-7890
fn create_phone_number(numbers:&[u8])->String{
    let number:Vec<_>= numbers.iter().map(|x| x.to_string()).collect();
    format!("({}) {}-{}",number[0..3].join(""),number[3..6].join(""),number[6..10].join(""))
}
