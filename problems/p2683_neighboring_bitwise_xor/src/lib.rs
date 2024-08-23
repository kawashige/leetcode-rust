pub struct Solution {}

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut org = 0;
        for i in 0..derived.len() - 1 {
            org ^= derived[i];
        }

        if org ^ derived[derived.len() - 1] == 0 {
            return true;
        }

        let mut org = 1;
        for i in 0..derived.len() - 1 {
            org ^= derived[i];
        }
        org ^ derived[derived.len() - 1] == 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2683() {
        assert!(Solution::does_valid_array_exist(vec![1, 1, 0]));
        assert!(Solution::does_valid_array_exist(vec![1, 1]));
        assert!(!Solution::does_valid_array_exist(vec![1, 0]));
    }
}
