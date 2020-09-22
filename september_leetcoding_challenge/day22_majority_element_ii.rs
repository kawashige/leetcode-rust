pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 2 && nums[0] == nums[1] {
            return vec![nums[0]];
        } else if nums.len() < 3 {
            return nums;
        }

        let floor = nums.len() / 3;
        let mut results = Vec::new();
        for i in 0..nums.len() {
            if results.contains(&nums[i])
                || !nums[(i + 1)..].contains(&nums[i])
                || nums[..i].contains(&nums[i])
            {
                continue;
            }
            let mut count = 1;
            for j in (i + 1)..nums.len() {
                if nums[i] == nums[j] {
                    count += 1;
                    if count > floor {
                        results.push(nums[i]);
                        if results.len() > 2 {
                            return results;
                        }
                        break;
                    }
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day22() {
        assert_eq!(vec![1], Solution::majority_element(vec![1]));
        assert_eq!(vec![1], Solution::majority_element(vec![1, 1]));
        assert_eq!(vec![1, 2], Solution::majority_element(vec![1, 2]));
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::majority_element(vec![3, 2, 1])
        );
        assert_eq!(vec![3], Solution::majority_element(vec![3, 2, 3]));
        assert_eq!(
            vec![1, 2],
            Solution::majority_element(vec![1, 1, 1, 3, 3, 2, 2, 2])
        );
    }
}
