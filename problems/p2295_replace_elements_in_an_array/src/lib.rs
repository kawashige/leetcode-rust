pub struct Solution {}

impl Solution {
    pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
        let mut indices = vec![-1; 1_000_001];
        for i in 0..nums.len() {
            indices[nums[i] as usize] = i as i32;
        }

        let mut nums = nums;
        for op in operations {
            nums[indices[op[0] as usize] as usize] = op[1];
            indices[op[1] as usize] = indices[op[0] as usize];
        }

        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2295() {
        assert_eq!(
            Solution::array_change(
                vec![1],
                vec![vec![1, 2], vec![2, 3], vec![3, 1000000], vec![1000000, 1]]
            ),
            vec![1]
        );
        assert_eq!(
            Solution::array_change(vec![1, 2, 4, 6], vec![vec![1, 3], vec![4, 7], vec![6, 1]]),
            vec![3, 2, 7, 1]
        );
        assert_eq!(
            Solution::array_change(vec![1, 2], vec![vec![1, 3], vec![2, 1], vec![3, 2]]),
            vec![2, 1]
        );
    }
}
