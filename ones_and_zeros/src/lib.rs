#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_correct() {
        assert_eq!(binary_slice_to_number(&[0, 0, 0, 1]), 1);
        assert_eq!(binary_slice_to_number(&[0, 0, 1, 0]), 2);
        assert_eq!(binary_slice_to_number(&[0, 1, 0, 1]), 5);
        assert_eq!(binary_slice_to_number(&[1, 0, 0, 1]), 9);
        assert_eq!(binary_slice_to_number(&[0, 0, 1, 0]), 2);
        assert_eq!(binary_slice_to_number(&[0, 1, 1, 0]), 6);
        assert_eq!(binary_slice_to_number(&[1, 1, 1, 1]), 15);
        assert_eq!(binary_slice_to_number(&[1, 0, 1, 1]), 11);
    }
}

fn binary_slice_to_number(slice:&[u32])->u32{
    slice.iter().fold(0,|acc,x| acc*2+x)
}
