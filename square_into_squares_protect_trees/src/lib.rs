#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: Option<Vec<i64>>) -> () {
        assert_eq!(decompose(n), exp)
    }
    
    #[test]
    fn tests_decompose() {
        
        testing(50, Some(vec![1,3,5,8,49]));
        testing(44, Some(vec![2,3,5,7,43]));
        testing(625, Some(vec![2,5,8,34,624]));
        testing(5, Some(vec![3,4]));
        
    }
}

fn decompose(n: i64) -> Option<Vec<i64>> {
    // your code
}
//recursive method
// 1^2 = None
// n^2 = k^2 + delta1
// (n+k)(n-k) = m^2 + delta2
// may use deep search method.