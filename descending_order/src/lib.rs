#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_correct() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
    }
}
// # Best Practice Version
// let mut result = x.to_string().chars().collect::<Vec<char>>();
// result.sort_by(|a, b| b.cmp(a));
// String::from_iter(result).parse::<u64>().unwrap()
// //almost the same
// //it's less optimal than 
// //a more mathy solution without doing string conversion
// another mathy solution use the iter::fold fn,
// like this:
// digits.iter().rev().fold(0, |s, n| s * 10 + n)
fn descending_order(x:u64)->u64{
    let mut x : Vec<_> = x.to_string().chars().collect();
    x.sort();
    x.reverse();
    let x :Vec<_> = x.iter().map(|&z| z.to_string()).collect();
    x.join("").parse().unwrap_or_else(|err| 0)
}
