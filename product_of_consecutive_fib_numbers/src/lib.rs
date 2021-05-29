
#[cfg(test)]
mod tests {
    use super::*;
    fn dotest(prod: u64, exp: (u64, u64, bool)) -> () {
        assert_eq!(product_fib(prod), exp)
    }
    
    #[test]
    fn basics_product_fib() {
        dotest(4895, (55, 89, true));
        dotest(5895, (89, 144, false));
        dotest(111114895, (55, 89, true));
        dotest(123165465895, (89, 144, false));
    }
}
fn product_fib(prod:u64)->(u64,u64,bool){
    //fnid all fib number which less than sqrt(prod)
    let prod_f = prod as f64;
    let upthres = prod_f.sqrt() as usize;//just big enough
    let mut fib_vec = Vec::new();
    for val in 0..upthres {
        let tt = fib_number(val, &fib_vec);
        fib_vec.push(tt);
        if tt > (upthres as u64){
            break;
        }
    }
    
    let result = fib_vec.iter().fold((0,0,false),|acc,&x| {
        if acc.2 {
            return acc
        }
        //cal old_val and new_val to find the edge
        let new_val = acc.1 * x;
        let old_val = acc.0 * acc.1;
        if new_val == prod || old_val < prod && new_val > prod{
            return (acc.1,x,true);
        }else{
            return  (acc.1,x,false);
        }
    });
    //verify and modify the result
    let result = if result.0 * result.1 == prod {
        result
    }else if result.0 * result.1 < prod{
        (result.1,fib_number(fib_vec.len(), &fib_vec),false)
    }else{
        (result.0,result.1,false)
    };
    result
}
fn fib_number(index:usize,fib_vec:&Vec<u64>)->u64{
    if index == 0 || index == 1 {
        1
    }else{
        let fib_n_2 = if let Some(res) = fib_vec.get(index-2){
            *res
        }else{
            fib_number(index-2, fib_vec)
        };
        let fib_n_1 = if let Some(res) = fib_vec.get(index-1){
            *res
        }else{
            fib_number(index-1, fib_vec)
        };
        fib_n_1.wrapping_add(fib_n_2)
    }
}
// my solution is stupid...I just calulated all fib numbers...
// just need 2 fib number, so it's easy to store them in 2 vars.
fn product_fib_example(prod: u64) -> (u64, u64, bool) {
    let mut a = 0;
    let mut b = 1;
    while a * b < prod {
        let c = a + b;
        a = b;
        b = c;
    }
    (a, b, a * b == prod)
}