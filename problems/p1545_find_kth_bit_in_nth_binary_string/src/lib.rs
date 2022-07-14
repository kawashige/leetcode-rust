pub struct Solution {}

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }

        let half = 2_i32.pow(n as u32 - 1);
        if k == half {
            '1'
        } else if k < half {
            Self::find_kth_bit(n - 1, k)
        } else {
            if Self::find_kth_bit(n - 1, half * 2 - k) == '0' {
                '1'
            } else {
                '0'
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1545() {
        assert_eq!(Solution::find_kth_bit(20, 1), '0');
        assert_eq!(Solution::find_kth_bit(3, 1), '0');
        assert_eq!(Solution::find_kth_bit(4, 11), '1');
    }
}
