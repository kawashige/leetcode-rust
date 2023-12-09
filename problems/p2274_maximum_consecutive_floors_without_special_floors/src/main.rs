pub struct Solution {}

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let mut special = special;
        special.push(bottom - 1);
        special.push(top + 1);
        special.sort_unstable();

        (1..special.len())
            .map(|i| special[i] - 1 - special[i - 1])
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2274() {
        assert_eq!(Solution::max_consecutive(2, 9, vec![4, 6]), 3);
        assert_eq!(Solution::max_consecutive(6, 8, vec![7, 6, 8]), 0);
    }
}
