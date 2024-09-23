pub struct Solution {}

impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let mut nums = nums;
        let mut min_cost = nums.iter().map(|num| *num as i64).sum::<i64>();
        let l = nums.len();
        for i in 0..l {
            let mut cost = (i as i64 + 1) * x as i64;
            let mut new_nums = nums.clone();
            for j in 0..l {
                new_nums[j] = nums[(j + l - 1) % l].min(new_nums[j]);
                cost += new_nums[j] as i64;
            }
            nums = new_nums;
            min_cost = min_cost.min(cost);
        }
        min_cost
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2735() {
        assert_eq!(Solution::min_cost(vec![15, 150, 56, 69, 214, 203], 42), 298);
        assert_eq!(Solution::min_cost(vec![20, 1, 15], 5), 13);
        assert_eq!(Solution::min_cost(vec![1, 2, 3], 4), 6);
    }
}
