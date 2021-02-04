pub struct Solution {}

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }
            for j in (i + 1)..nums.len() {
                if nums[j] == 0 {
                    continue;
                }
                let min = nums[j] - nums[i];
                let max = nums[j] + nums[i];
                count += ((j + 1)..nums.len())
                    .take_while(|k| min < nums[*k] && nums[*k] < max)
                    .count()
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0611() {
        assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    }
}
