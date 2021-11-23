pub struct Solution {}

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }

        let mut i = 1;
        let mut num = 1;
        while num < n {
            i *= 4;
            num += i;
        }

        let mut c = vec!['1'];
        i /= -2;
        while i != 0 {
            if i > 0 {
                if num - i >= n {
                    num -= i;
                    c.push('0');
                } else {
                    c.push('1');
                }
            } else if i < 0 {
                if num + i >= n {
                    num += i;
                    c.push('1');
                } else {
                    c.push('0');
                }
            }
            i /= -2;
        }

        c.iter().collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1017() {
        assert_eq!(Solution::base_neg2(30), "1100010".to_string());
        assert_eq!(Solution::base_neg2(2), "110".to_string());
        assert_eq!(Solution::base_neg2(3), "111".to_string());
        assert_eq!(Solution::base_neg2(4), "100".to_string());
    }
}
