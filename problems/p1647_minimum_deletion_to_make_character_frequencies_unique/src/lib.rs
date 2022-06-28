pub struct Solution {}

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut count = s
            .as_bytes()
            .iter()
            .fold(vec![0; 26], |mut count, b| {
                count[(b - b'a') as usize] += 1;
                count
            })
            .into_iter()
            .filter(|c| &0 < c)
            .collect::<Vec<_>>();

        count.sort_unstable();

        let mut used = *count.last().unwrap();
        let mut deletion = 0;
        for x in count[..count.len() - 1].iter().rev() {
            if used == 0 {
                deletion += x;
            } else if x < &used {
                used = *x;
            } else {
                used -= 1;
                deletion += x - used;
            }
        }

        deletion
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1647() {
        assert_eq!(Solution::min_deletions("aab".to_string()), 0);
        assert_eq!(Solution::min_deletions("aaabbbcc".to_string()), 2);
        assert_eq!(Solution::min_deletions("ceabaacb".to_string()), 2);
    }
}
