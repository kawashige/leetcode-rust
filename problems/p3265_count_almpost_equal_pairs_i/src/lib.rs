pub struct Solution {}

impl Solution {
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let nums = nums
            .into_iter()
            .map(|n| format!("{:07}", n))
            .collect::<Vec<_>>();

        for i in 0..nums.len() {
            for j in 0..i {
                let mut diff = Vec::new();
                for k in 0..nums[i].len() {
                    if nums[i].as_bytes()[k] != nums[j].as_bytes()[k] {
                        diff.push(k);
                    }
                }
                if diff.len() == 0 {
                    count += 1;
                } else if diff.len() == 2
                    && nums[i].as_bytes()[diff[0]] == nums[j].as_bytes()[diff[1]]
                    && nums[i].as_bytes()[diff[1]] == nums[j].as_bytes()[diff[0]]
                {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3265() {
        assert_eq!(Solution::count_pairs(vec![3, 12, 30, 17, 21]), 2);
        assert_eq!(Solution::count_pairs(vec![1, 1, 1, 1, 1]), 10);
        assert_eq!(Solution::count_pairs(vec![123, 231]), 0);
    }
}
