pub struct Solution {}

impl Solution {
    pub fn even_odd_bit(n: i32) -> Vec<i32> {
        let mut result = vec![0, 0];
        for i in 0..11 {
            if n & 1 << i != 0 {
                result[i % 2] += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2595() {
        assert_eq!(Solution::even_odd_bit(50), [1, 2]);
        assert_eq!(Solution::even_odd_bit(2), [0, 1]);
    }
}
