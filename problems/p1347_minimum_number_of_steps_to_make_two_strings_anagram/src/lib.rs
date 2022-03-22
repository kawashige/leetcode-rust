pub struct Solution {}

impl Solution {
    pub fn char_count(s: &str) -> [i32; 26] {
        s.chars().fold([0; 26], |mut count, c| {
            count[c as usize - 0x61] += 1;
            count
        })
    }

    pub fn min_steps(s: String, t: String) -> i32 {
        Self::char_count(&s)
            .iter()
            .zip(Self::char_count(&t).iter())
            .fold(0, |count, (s_count, t_count)| {
                count + (s_count - t_count).abs()
            })
            / 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1347() {
        assert_eq!(Solution::min_steps("bab".to_string(), "aba".to_string()), 1);
        assert_eq!(
            Solution::min_steps("leetcode".to_string(), "practice".to_string()),
            5
        );
        assert_eq!(
            Solution::min_steps("anagram".to_string(), "manager".to_string()),
            1
        );
    }
}
