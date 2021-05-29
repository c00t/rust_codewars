#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(in_array(
            &["xyz", "live", "strong"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["live", "strong"]);
        
        assert_eq!(in_array(
            &["live", "strong", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);
        
        assert_eq!(in_array(
            &["tarp", "mice", "bull"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), [] as [&str; 0]);
        
        assert_eq!(in_array(
            &["live", "strong", "arp", "arp"],
            &["lively", "alive", "harp", "sharp", "armstrong"],
        ), ["arp", "live", "strong"]);
    }
}
fn in_array(arr_a:&[&str],arr_b:&[&str])->Vec<String>{
    let mut result = Vec::new();
    for &des in arr_a.iter() {
        for &vsdes in arr_b.iter(){
            if vsdes.contains(des) {
                result.push(des.to_string());
                break;
            }
        }
    }
    result.sort();//first sort to eliminate the order difference in arr_a
    result.dedup();//then remove [consecutive] repeats, because [sort], it's correct
    result
}
// one example in Codewars
// core concept in this example is filter(|&e| arr_b.iter().any(|&t| t.contains(e)))
fn in_array_2(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = arr_a.iter()
        .filter(|&e| arr_b.iter().any(|&t| t.contains(e)))
        .map(|s| s.to_string())
        .collect();
    
    result.sort_unstable();
    result.dedup();
    result
}