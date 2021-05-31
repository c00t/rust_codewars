#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!("-6,-3-1,3-5,7-11,14,15,17-20", solution::range_extraction(&[-6,-3,-2,-1,0,1,3,4,5,7,8,9,10,11,14,15,17,18,19,20]));	
        assert_eq!("-3--1,2,10,15,16,18-20", solution::range_extraction(&[-3,-2,-1,2,10,15,16,18,19,20]));
    }
}

mod solution {
    
    pub fn range_extraction(a: &[i32]) -> String {
        // Your solution here
        let mut a = a.to_owned();
        a.sort();

        let mut range_result : Vec<Vec<i32>> = Vec::new();
        let mut old = 0;
        let mut temp:Vec<i32> = Vec::new();
        // can use iter().enumerate() here.
        for index in 0..a.len() {
            if index != 0 {
                if a[index] == old + 1 {
                    temp.push(a[index]);
                }else{
                    range_result.push(temp.to_owned());
                    temp.clear();
                    temp.push(a[index]);
                }
                old = a[index];
            }else{
                temp.push(a[index]);
                old = a[index];
            }
        }
        if temp.len() != 0 {
            range_result.push(temp.to_owned());
        }
        range_result.iter().map(|v| {
            if v.len() <= 2 {
                v.iter().map(|i| {
                    i.to_string()
                }).collect::<Vec<String>>().join(",")
            }else{
                format!("{}-{}",v[0],v[v.len()-1])
            }
        }).collect::<Vec<String>>().join(",")
    }


    ////////// Other Examples
    // Remember to use unwrap when Option<T>
    // use itertools::Itertools;
  
    // // https://stackoverflow.com/a/50392400
    // fn consecutive_slices(data: &[i32]) -> Vec<Vec<i32>> {
    //   (&(0..data.len()).group_by(|&i| (data[i] as usize).wrapping_sub(i)))
    //     .into_iter()
    //     .map(|(_, group)| group.map(|i| data[i]).collect())
    //     .collect()
    // }
    
    // fn vec_to_string(vec: &Vec<i32>) -> String {
    //   match &vec[..] {
    //     &[] => "".to_string(),
    //     &[a] => a.to_string(),
    //     &[a, b] => format!("{},{}", a, b),
    //     _ => format!("{}-{}", vec[0], vec.iter().last().unwrap())
    //   }
    // }
    
    // pub fn range_extraction(a: &[i32]) -> String {
    //   consecutive_slices(a).iter().map(vec_to_string).collect::<Vec<_>>().join(",")
    // }

    ////////// Other Example [Best Practice]
    // pub fn range_extraction(a: &[i32]) -> String {
    //     let mut ans = vec![];
    //     let mut start = a[0];
    //     let mut current = a[0];
    //     // use skip to skip items
    //     // and skip more if-else branch, there is [no need to memo every value]. 
    //     // we only need the edge value.
    //     for i in a.iter().skip(1) {
    //         if *i != current + 1 {
    //             ans.push((start, current));
    //             start = *i;
    //         }
    //         current = *i;
    //     }
    //     ans.push((start, current));
    //     ans.iter()
    //         .map(|(start, end)| {
    //             if end == start {
    //                 end.to_string()
    //             } else if *end == start + 1 {
    //                 format!("{},{}", start, end)
    //             } else {
    //                 format!("{}-{}", start, end)
    //             }
    //         })
    //         .collect::<Vec<_>>()
    //         .join(",")
    // }

}