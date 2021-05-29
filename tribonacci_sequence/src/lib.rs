#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_tests() {
        assert_eq!(tribonacci(&[0., 1., 1.], 10), vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]);
        assert_eq!(tribonacci(&[1., 0., 0.], 10), vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]);
        assert_eq!(tribonacci(&[0., 0., 0.], 10), vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]);
        assert_eq!(tribonacci(&[1., 2., 3.], 10), vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]);
        assert_eq!(tribonacci(&[3., 2., 1.], 10), vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]);
        assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
        assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
        assert_eq!(tribonacci(&[0.5, 0.5, 0.5], 30), vec![0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5, 1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5, 266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5]);;
    }
}
fn nacci_number(index:usize,arr_sign:&[f64; 3],fib_vec:&Vec<f64>)->f64{
    if index == 0 || index == 1 || index == 2 {
        arr_sign[index]
    }else{
        let fib_n_3 = if let Some(res) = fib_vec.get(index-3){
            *res
        }else{
            nacci_number(index-3, arr_sign,fib_vec)
        };
        let fib_n_2 = if let Some(res) = fib_vec.get(index-2){
            *res
        }else{
            nacci_number(index-2, arr_sign,fib_vec)
        };
        let fib_n_1 = if let Some(res) = fib_vec.get(index-1){
            *res
        }else{
            nacci_number(index-1, arr_sign,fib_vec)
        };
        fib_n_1+fib_n_2+fib_n_3
    }
}
fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    // your code here
    let mut result = Vec::new();
    for index in 0..n {//remember that n is not included!!!

        let tt = nacci_number(index, signature,&result);
        result.push(tt);
    }
    result
}

// example
// because the "product_of_consecutive_fib_numbers" kata, just use the method
// but see below,
// example is a more concise version, just use Vec, first enlarge the vector, then use it as cache
fn tribonacci_vec_cache(signature: &[f64], n: usize) -> Vec<f64> {
    let mut cache = signature.to_vec();
    
    cache.resize(n, 0.0);
    
    for i in 3..cache.len() {
        cache[i] = cache[i - 1] + cache[i - 2] + cache[i - 3];
    }
    
    cache
}