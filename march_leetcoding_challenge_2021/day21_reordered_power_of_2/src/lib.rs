pub struct Solution {}

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        fn recurse(chars: &[char], used: &mut Vec<bool>, s: &mut String) -> bool {
            if chars.len() == s.len() {
                let n = s.parse::<usize>().unwrap();
                return n & (n - 1) == 0;
            }

            for i in 0..chars.len() {
                if used[i] || (s.is_empty() && chars[i] == '0') {
                    continue;
                }

                used[i] = true;
                s.push(chars[i]);
                if recurse(chars, used, s) {
                    return true;
                }
                used[i] = false;
                s.pop();
            }
            false
        }

        let chars = n.to_string().chars().collect::<Vec<char>>();
        let mut used = vec![false; chars.len()];
        recurse(&chars, &mut used, &mut String::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day21() {
        assert!(Solution::reordered_power_of2(1));
        assert!(!Solution::reordered_power_of2(10));
        assert!(Solution::reordered_power_of2(16));
        assert!(!Solution::reordered_power_of2(24));
        assert!(Solution::reordered_power_of2(46));
    }
}
