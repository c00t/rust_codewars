
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_sudoku() {
        let good_sudoku_1 = Sudoku{
            data: vec![
                    vec![7,8,4, 1,5,9, 3,2,6],
                    vec![5,3,9, 6,7,2, 8,4,1],
                    vec![6,1,2, 4,3,8, 7,5,9],
    
                    vec![9,2,8, 7,1,5, 4,6,3],
                    vec![3,5,7, 8,4,6, 1,9,2],
                    vec![4,6,1, 9,2,3, 5,8,7],
                    
                    vec![8,7,6, 3,9,4, 2,1,5],
                    vec![2,4,3, 5,6,1, 9,7,8],
                    vec![1,9,5, 2,8,7, 6,3,4]
                ]
        };
        
        let good_sudoku_2 = Sudoku{
            data: vec![
                    vec![1, 4,  2, 3],
                    vec![3, 2,  4, 1],
            
                    vec![4, 1,  3, 2],
                    vec![2, 3,  1, 4],
                ]
        };
        assert!(good_sudoku_1.is_valid());
        assert!(good_sudoku_2.is_valid());
    }
    
    #[test]
    fn bad_sudoku() {
        let bad_sudoku_1 = Sudoku{
            data: vec![
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
    
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                    vec![1,2,3, 4,5,6, 7,8,9],
                ]
        };
        
        let bad_sudoku_2 = Sudoku{
            data: vec![
                    vec![1,2,3,4,5],
                    vec![1,2,3,4],
                    vec![1,2,3,4],
                    vec![1],
                ]
        };
        assert!(!bad_sudoku_1.is_valid());
        assert!(!bad_sudoku_2.is_valid());
    }
}

struct Sudoku{
    data: Vec<Vec<u32>>,
}

impl Sudoku{
    fn is_valid(&self) -> bool {
      // YOUR SOLUTION
        let square_size = self.data.len() as f64;
        let square_size = square_size.sqrt() as u32;
        let mut notation = Vec::new();
        let size = self.data.len();
        notation.resize(size, 0);
        println!("passed check 0.");
        //check all index equal
        if !self.data.iter().all(|x| x.len()==size) {
            return false;
        }
        println!("passed check 1.");
        //check all element in range
        if self.data.iter().any(|row| {
            row.iter().any(|x| *x < 1 || *x > (size as u32))
        }) {
            return false;
        }
        //check every row
        for row in 0..size {
            notation.fill(0);
            for colomn in 0..size {
                let element = self.data[row][colomn] - 1;
                notation[element as usize] += 1;
            }
            if notation.iter().all(|&x| x == 1) {
                continue;
            }else {
                return false;
            }
        }
        println!("passed check 2.");
        //check every colomn
        for colomn in 0..size {
            notation.fill(0);
            for row in 0..size {
                let element = self.data[row][colomn] - 1;
                notation[element as usize] += 1;
            }
            if notation.iter().all(|&x| x == 1) {
                continue;
            }else {
                return false;
            }
        }
        println!("passed check 3.");
        //check every square
        for row in 0..square_size { //0..3
            for colomn in 0..square_size { //0..3
                notation.fill(0);
                for i in 0..size{ //0..9
                    //get actual x&y
                    let x = ((i as u32)/(square_size as u32) + row*square_size) as usize;
                    let y = ((i as u32)%(square_size as u32) + colomn*square_size) as usize;
                    //println!("{},{}",x,y);
                    let element = self.data[x][y] -1;
                    notation[element as usize] += 1;
                }
                if notation.iter().all(|&x| x == 1) {
                    continue;
                }else {
                    return false;
                }
            }
        }
        println!("passed check 4.");
        true
    }
}