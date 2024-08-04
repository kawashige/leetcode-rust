pub struct Solution {}

impl Solution {
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if nums.iter().any(|num| num == &1) {
            return nums.len() as i32 - nums.iter().filter(|num| num == &&1).count() as i32;
        }

        let mut min_count = std::i32::MAX;
        for i in 0..nums.len() {
            let mut num = nums[i];
            for j in i + 1..nums.len() {
                num = Self::gcd(num, nums[j]);
                if num == 1 {
                    min_count = min_count.min((j - i) as i32);
                    break;
                }
            }
        }
        if min_count == std::i32::MAX {
            return -1;
        }

        min_count + nums.len() as i32 - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2654() {
        assert_eq!(Solution::min_operations(vec![6, 10, 15]), 4);
        assert_eq!(Solution::min_operations(vec![2, 6, 3, 4]), 4);
        assert_eq!(Solution::min_operations(vec![2, 10, 6, 14]), -1);
    }
}
