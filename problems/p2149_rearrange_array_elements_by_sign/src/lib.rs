pub struct Solution {}

impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut positive = 0;
        let mut negative = 1;
        let mut result = vec![0; nums.len()];

        for num in nums {
            if 0 < num {
                result[positive] = num;
                positive += 2;
            } else {
                result[negative] = num;
                negative += 2;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2149() {
        assert_eq!(
            Solution::rearrange_array(vec![3, 1, -2, -5, 2, -4]),
            vec![3, -2, 1, -5, 2, -4]
        );
        assert_eq!(Solution::rearrange_array(vec![-1, 1]), vec![1, -1]);
    }
}
