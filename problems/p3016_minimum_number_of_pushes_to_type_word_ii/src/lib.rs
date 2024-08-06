pub struct Solution {}

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut count = word.as_bytes().iter().fold(vec![0; 26], |mut count, b| {
            count[(b - b'a') as usize] += 1;
            count
        });
        count.sort_unstable_by_key(|c| *c as i32 * -1);
        count
            .into_iter()
            .enumerate()
            .map(|(i, c)| (((i / 8) + 1) * c) as i32)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3016() {
        assert_eq!(Solution::minimum_pushes("abcde".to_string()), 5);
        assert_eq!(Solution::minimum_pushes("xyzxyzxyzxyz".to_string()), 12);
        assert_eq!(
            Solution::minimum_pushes("aabbccddeeffgghhiiiiii".to_string()),
            24
        );
    }
}
