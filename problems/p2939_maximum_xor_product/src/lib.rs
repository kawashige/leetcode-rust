pub struct Solution {}

impl Solution {
    pub fn maximum_xor_product(a: i64, b: i64, n: i32) -> i32 {
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        for i in n..=64 - a.leading_zeros() as i32 {
            if a & 1 << i != 0 {
                x |= 1 << i
            }
        }
        for i in n..=64 - b.leading_zeros() as i32 {
            if b & 1 << i != 0 {
                y |= 1 << i
            }
        }
        for i in (0..n).rev() {
            if a & 1 << i == b & 1 << i {
                x |= 1 << i;
                y |= 1 << i;
            } else {
                if x <= y {
                    x |= 1 << i;
                } else {
                    y |= 1 << i;
                }
            }
        }
        ((x as i128 * y as i128) % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2939() {
        assert_eq!(Solution::maximum_xor_product(0, 7, 2), 12);
        assert_eq!(Solution::maximum_xor_product(0, 3, 1), 2);
        assert_eq!(Solution::maximum_xor_product(12, 5, 4), 98);
        assert_eq!(Solution::maximum_xor_product(6, 7, 5), 930);
        assert_eq!(Solution::maximum_xor_product(1, 6, 3), 12);
    }
}
