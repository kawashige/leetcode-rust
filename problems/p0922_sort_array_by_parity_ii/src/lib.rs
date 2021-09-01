pub struct Solution {}

impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let mut even = 0;
        let mut odd = 1;

        while even < nums.len() {
            while even < nums.len() && nums[even] % 2 == 0 {
                even += 2;
            }
            while odd < nums.len() && nums[odd] % 2 == 1 {
                odd += 2;
            }

            if even < nums.len() {
                nums.swap(even, odd);
            }
        }

        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0922() {
        assert_eq!(
            Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
            vec![4, 5, 2, 7]
        );
        assert_eq!(Solution::sort_array_by_parity_ii(vec![2, 3]), vec![2, 3]);
    }
}
