pub struct Solution {}

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let n = encoded.len() + 1;
        let all_xor = (1..=n).fold(0, |acc, x| acc ^ x as i32);
        let odd_xor = (1..encoded.len())
            .step_by(2)
            .fold(0, |acc, i| acc ^ encoded[i]);

        let mut result = vec![0; n];
        result[0] = all_xor ^ odd_xor;
        for i in 1..n {
            result[i] = result[i - 1] ^ encoded[i - 1];
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1734() {
        assert_eq!(Solution::decode(vec![3, 1]), vec![1, 2, 3]);
        assert_eq!(Solution::decode(vec![6, 5, 4, 6]), vec![2, 4, 1, 5, 3]);
    }
}
