#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(count_bits(4), 1);
        assert_eq!(count_bits(7), 3);
        assert_eq!(count_bits(9), 2);
        assert_eq!(count_bits(10), 2);
    }
}
fn count_bits(n: i64) -> u32 {
    // code here
    // use rust, easy practice
    // maybe Rust std library includes too many functions?
    n.count_ones()

    // some format! macro usage
    // format!("{:b}", n).matches('1').count() as u32
}