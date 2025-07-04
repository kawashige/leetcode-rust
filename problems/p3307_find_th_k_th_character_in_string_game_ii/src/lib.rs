pub struct Solution {}

impl Solution {
    pub fn recurse(k: i64, operations: &[i32]) -> (i64, i32) {
        let mut i = 1;
        let mut j = 0;
        while i < k {
            i *= 2;
            j += 1;
        }
        let k = k - i / 2;
        (k, if j == 0 { 0 } else { operations[j - 1] })
    }

    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut k = k;
        let mut c = 0;
        while 1 < k {
            let (new_k, count) = Self::recurse(k, &operations);
            k = new_k;
            c += count;
        }
        ((c % 26) as u8 + b'a') as char
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3307() {
        assert_eq!(Solution::kth_character(5, vec![0, 0, 0]), 'a');
        assert_eq!(Solution::kth_character(10, vec![0, 1, 0, 1]), 'b');
    }
}
