pub struct Solution {}

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        for i in 0..word.len() {
            let mut count = (0..word.len())
                .filter(|j| &i != j)
                .fold([0; 26], |mut count, j| {
                    count[(word.as_bytes()[j] - b'a') as usize] += 1;
                    count
                })
                .into_iter()
                .filter(|c| &0 < c)
                .collect::<Vec<_>>();
            count.sort_unstable();
            if &count[0] == count.last().unwrap() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2423() {
        assert!(Solution::equal_frequency("abcc".to_string()));
        assert!(!Solution::equal_frequency("aazz".to_string()));
    }
}
