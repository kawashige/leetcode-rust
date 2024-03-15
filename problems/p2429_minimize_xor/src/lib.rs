pub struct Solution {}

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut count = num2.count_ones() as usize;
        let mut num = 0;
        for i in (0..32).rev() {
            if num1 & 1 << i != 0 || count == i + 1 {
                num |= 1 << i;
                count -= 1;
                if count == 0 {
                    break;
                }
            }
        }
        num
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2429() {
        assert_eq!(Solution::minimize_xor(3, 5), 3);
        assert_eq!(Solution::minimize_xor(1, 12), 3);
    }
}
