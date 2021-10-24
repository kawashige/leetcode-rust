pub struct Solution {}

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        if a >= b {
            let mut v = vec![String::new(); (a as usize + 1) / 2];
            let mut used_a = 0;
            while used_a < a {
                for i in 0..v.len() {
                    if used_a == a {
                        break;
                    }
                    v[i].push('a');
                    used_a += 1;
                }
            }
            let mut used_b = 0;
            while used_b < b {
                for i in 0..v.len() {
                    if used_b == b {
                        break;
                    }
                    v[i].push('b');
                    used_b += 1;
                }
            }
            v.into_iter().collect()
        } else {
            let mut v = vec![String::new(); (b as usize + 1) / 2];
            let mut used_a = 0;
            let mut used_b = 0;
            while used_b < b {
                for i in 0..v.len() {
                    if used_b == b {
                        break;
                    }
                    v[i].push('b');
                    used_b += 1;
                }
            }
            while used_a < a {
                for i in 0..v.len() {
                    if used_a == a {
                        break;
                    }
                    v[i].push('a');
                    used_a += 1;
                }
            }
            v.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0984() {
        assert_eq!(Solution::str_without3a3b(4, 4), "aabbaabb".to_string());
        assert_eq!(Solution::str_without3a3b(3, 4), "bbaabba".to_string());
        assert_eq!(Solution::str_without3a3b(3, 3), "aabbab".to_string());
        assert_eq!(Solution::str_without3a3b(2, 5), "bbabbab".to_string());
        assert_eq!(Solution::str_without3a3b(1, 3), "bbab".to_string());
        assert_eq!(Solution::str_without3a3b(1, 1), "ab".to_string());
        assert_eq!(Solution::str_without3a3b(1, 2), "bba".to_string());
        assert_eq!(Solution::str_without3a3b(4, 1), "aabaa".to_string());
    }
}
