pub struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let t_count = t.chars().fold(vec![0; 128], |mut count, c| {
            count[c as usize] += 1;
            count
        });
        let mut char_count = t_count.iter().filter(|x| x > &&0).count();

        let mut s_count = vec![0; 128];
        let s_chars = s.chars().collect::<Vec<_>>();
        let mut i = 0;
        let mut j = 0;
        while j < s_chars.len() && char_count > 0 {
            s_count[s_chars[j] as usize] += 1;
            if s_count[s_chars[j] as usize] == t_count[s_chars[j] as usize] {
                char_count -= 1;
            }
            j += 1;
        }

        if char_count > 0 {
            return Default::default();
        }

        while t_count[s_chars[i] as usize] == 0
            || s_count[s_chars[i] as usize] > t_count[s_chars[i] as usize]
        {
            s_count[s_chars[i] as usize] -= 1;
            i += 1;
        }

        let mut min_i = i;
        let mut min_j = j - 1;

        for k in j..s_chars.len() {
            s_count[s_chars[k] as usize] += 1;
            while t_count[s_chars[i] as usize] == 0
                || s_count[s_chars[i] as usize] > t_count[s_chars[i] as usize]
            {
                s_count[s_chars[i] as usize] -= 1;
                i += 1;
            }

            if min_j - min_i > k - i {
                min_j = k;
                min_i = i;
            }
        }

        s_chars[min_i..=min_j].iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day15() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "a".to_string()),
            "a".to_string()
        );
        assert_eq!(
            Solution::min_window("a".to_string(), "aa".to_string()),
            "".to_string()
        );
        assert_eq!(
            Solution::min_window("ab".to_string(), "a".to_string()),
            "a".to_string()
        );
    }
}
