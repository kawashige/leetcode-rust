pub struct Solution {}

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut counts = vec![0_usize; 32];
        counts[0] = 1;
        for i in 1..counts.len() {
            counts[i] = counts[i - 1] * 2 + 1;
        }

        for i in (0..32).rev() {
            if n & 1 << i != 0 {
                if i == 0 {
                    return counts[0] as i32;
                } else {
                    return (counts[i]
                        - Self::minimum_one_bit_operations(n & (2_i32.pow(i as u32) - 1)) as usize)
                        as i32;
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1611() {
        assert_eq!(Solution::minimum_one_bit_operations(2480), 3807);
        assert_eq!(Solution::minimum_one_bit_operations(9), 14);
        assert_eq!(Solution::minimum_one_bit_operations(3), 2);
        assert_eq!(Solution::minimum_one_bit_operations(6), 4);
    }
}
