pub struct Solution {}

impl Solution {
    pub fn is_digitorial_permutation(n: i32) -> bool {
        let mut factorial = [1; 10];
        for i in 1..factorial.len() {
            factorial[i] = factorial[i - 1] * i as i32;
        }

        let mut original_digits = [0; 10];
        let mut sum = 0;
        for c in n.to_string().as_bytes().iter() {
            let d = (c - b'0') as usize;
            original_digits[d] += 1;
            sum += factorial[d];
        }

        let mut digits = [0; 10];
        for c in sum.to_string().as_bytes().iter() {
            let d = (c - b'0') as usize;
            digits[d] += 1;
        }

        (0..digits.len()).all(|i| original_digits[i] == digits[i])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3848() {
        assert!(Solution::is_digitorial_permutation(145));
        assert!(!Solution::is_digitorial_permutation(10));
    }
}

fn main() {}
