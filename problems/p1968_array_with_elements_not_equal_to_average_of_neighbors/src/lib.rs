pub struct Solution {}

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut rearranged = vec![0; nums.len()];

        let mut j = 0;
        for i in 0..nums.len() {
            rearranged[j] = nums[i];
            j += 2;
            if rearranged.len() <= j {
                j = 1;
            }
        }

        rearranged
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1968() {
        assert_eq!(
            Solution::rearrange_array(vec![1, 2, 3, 4, 5]),
            vec![1, 4, 2, 5, 3]
        );
        assert_eq!(
            Solution::rearrange_array(vec![6, 2, 0, 9, 7]),
            vec![0, 7, 2, 9, 6]
        );
        assert_eq!(
            Solution::rearrange_array(vec![6, 2, 0, 9]),
            vec![0, 6, 2, 9]
        );
    }
}
