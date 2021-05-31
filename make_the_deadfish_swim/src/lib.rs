#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"),
            vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"),
            vec![8, 64, 3600]);
    }
}
fn parse(code: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let mut temp:i32 = 0;
    for ch in code.chars() {
        match ch {
            'i' => {
                temp += 1;
            },
            'd' => {
                temp -= 1;
            },
            's' => {
                temp = temp.pow(2);
            },
            'o' => {
                result.push(temp);
                //temp = 0;
            },
            _ => {},
        };
    }
    result
}