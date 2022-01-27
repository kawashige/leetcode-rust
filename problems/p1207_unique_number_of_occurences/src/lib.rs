pub struct Solution {}

impl Solution {
    pub fn unique_occurrences(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();

        let mut occurences = vec![false; arr.len() + 1];
        let mut occurence = 0;

        for i in 0..arr.len() {
            occurence += 1;
            if i == arr.len() - 1 || arr[i] != arr[i + 1] {
                if occurences[occurence] {
                    return false;
                }
                occurences[occurence] = true;
                occurence = 0;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1207() {
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
        assert!(!Solution::unique_occurrences(vec![1, 2]));
        assert!(Solution::unique_occurrences(vec![
            -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
        ]));
    }
}
