#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}
fn persistence(num: u64) -> u64 {
    // if num < 10{
    //     return 0;
    // }
    let mut index = 0;
    multi_persist(num,&mut index);
    index
}
fn multi_persist(num:u64,index:&mut u64){
    if(num < 10) {
        return;
    }
    let result:u64 = num.to_string().chars().fold(1, |acc,x| {
        acc*x.to_digit(10).unwrap() as u64
    });
    *index += 1;
    multi_persist(result, index);
}
/*
// Some "Best practice"
// use String maybe performance-critical sometime
// In fact, it's a rem-product recursion.
pub fn persistence(num: u64) -> u64 {
    let mut n = num;
    let mut count = 0;
    while n > 9 {
        n = prod(n);
        count +=1;
    }
    count
}

fn prod(n: u64) -> u64 {
    let mut n = n;
    let mut prod = 1;
    while n > 0 {
        prod *= n%10;
        n /= 10;
    }
    prod
}

*/