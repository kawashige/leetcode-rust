pub struct Solution {}

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut count = [[-1; 3]; 26];
        let mut c = 0;
        for i in 0..s.len() {
            if i == 0 || s.as_bytes()[i] != s.as_bytes()[i - 1] {
                c = 1;
            } else {
                c += 1;
            }
            let k = (s.as_bytes()[i] - b'a') as usize;
            if count[k][0] < c {
                count[k][0] = c;
                for j in 0..2 {
                    if count[k][j + 1] < count[k][j] {
                        count[k].swap(j, j + 1);
                    }
                }
            }
        }
        (0..count.len()).map(|i| count[i][0]).max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2982() {
        assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
        assert_eq!(Solution::maximum_length("abcdef".to_string()), -1);
        assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
    }
}
