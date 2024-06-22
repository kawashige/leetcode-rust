pub struct Solution {}

impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(word.len());
        let mut remains = 0;
        let m = m as usize;

        for i in 0..word.len() {
            remains = (remains * 10 + (word.as_bytes()[i] - b'0') as usize) % m;
            result.push(if remains == 0 { 1 } else { 0 });
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2575() {
        assert_eq!(
            Solution::divisibility_array("998244353".to_string(), 3),
            vec![1, 1, 0, 0, 0, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::divisibility_array("1010".to_string(), 10),
            vec![0, 1, 0, 1]
        );
    }
}
