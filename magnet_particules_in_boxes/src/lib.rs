#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        println!("{}",doubles(1,10));
        println!("{}",doubles(10,1000));
        println!("{}",doubles(10,10000));
        assert_eq!(2 + 1, 4);
    }
    
}
// Below method will overflow
// fn doubles(maxk: i32, maxn: i32) -> f64{
//     let memo = (1..maxn+1).map(|x| x.pow(2)).collect::<Vec<i32>>();
//     (1..maxk+1).map(|k| {
//         (0..maxn).map(|index| {
//             1.0/((k*memo[index as usize].pow(k as u32)) as f64)
//         }).sum::<f64>()
//     }).sum()
// }


// v(k,n-1) = 
// sigma<n>v(k,n) = sigma<n-1>(v(k,n-1)) + 1/k*(N+1)^2k

fn doubles(maxk: i32, maxn: i32) -> f64{
    // maxk is outer bounds, maxn is inner bounds
    let mut memo = vec![vec![-1.0;maxn as usize];maxk as usize];
    for k in 0..maxk {
        for n in 0..maxn {
            let k_n:f64 = acc_cons_k_n((k+1) as u32,(n+1) as u32,&memo);
            memo[k as usize][n as usize] = k_n;
        }
    }
    //println!("{:?}",memo);
    memo.iter().map(|v| v.iter().last().unwrap()).sum()

}
fn acc_cons_k_n(k:u32,n:u32,memo:&Vec<Vec<f64>>) -> f64{
    //println!("{},{}",k,n);
    if n == 1 {
        let sum = (1..2*k+1).fold(1.0, |acc,x| {
            acc * 1.0/((2) as f64)
        });
        return 1.0/(k as f64) * sum;
    }
    if memo[(k-1) as usize][(n-1) as usize] != -1.0 {
        memo[(k-1) as usize][(n-1) as usize]
    }else{
        // should avoid overflow
        let sum = (1..2*k+1).fold(1.0, |acc,x| {
            acc * 1.0/((n+1) as f64)
        });
        acc_cons_k_n(k, n-1, memo) + 1.0/(k as f64) * sum
    }
}

// ... A Consise&Quick Version, mainly use other pow() function, use pow will overflow easily.
fn doubles_example(maxk: i32, maxn: i32) -> f64 {
    let v = |k: f64, n: f64|->f64 {1.0 / (k * (n + 1.0).powf(2.0 * k))};
    let u = |k, N|->f64 {(1..=N).map(|n| v(k as f64, n as f64)).sum()};
    let S = |K, N| (1..=K).map(|k| u(k, N)).sum();// A new range usage...
    
    S(maxk, maxn)
}
