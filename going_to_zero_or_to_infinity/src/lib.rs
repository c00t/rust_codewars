#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
// u1 = (1 / 1!) * (1!)
// u2 = (1 / 2!) * (1! + 2!)
// u3 = (1 / 3!) * (1! + 2! + 3!)
// ...
// un = (1 / n!) * (1! + 2! + 3! + ... + n!)
// Then un = ( 1/(n-1)!n ) * (1! + 2! +3! + ... + (n-1)! + n(n-1)!)
// Then un = (1/n)*u_n-1 + 1

fn going(n: i32) -> f64 {
    (((un(n)*1000000.0) as u32) as f64)/1000000.0
    // (res * 1e6).floor() / 1e6
    // 1e6 and floor() method
}
fn un(n:i32) -> f64{
    if n==1 {
        return 1.0
    }
    (1.0/n as f64)*un(n-1) + 1.0
}