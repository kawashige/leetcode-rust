pub struct Solution {}

impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut decoded = vec![0; encoded.len() + 1];
        decoded[0] = first;

        for i in 1..decoded.len() {
            decoded[i] = decoded[i - 1] ^ encoded[i - 1];
        }

        decoded
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1720() {
        assert_eq!(Solution::decode(vec![1, 2, 3], 1), vec![1, 0, 2, 1]);
        assert_eq!(Solution::decode(vec![6, 2, 7, 3], 4), vec![4, 2, 0, 7, 4]);
    }
}
