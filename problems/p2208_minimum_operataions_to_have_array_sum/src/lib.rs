use std::collections::BinaryHeap;

pub struct Solution {}

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut sum = nums[0] as u128;

        for i in 1..nums.len() {
            max = max.max(nums[i]);
            sum += nums[i] as u128;
        }

        let mut mul = 1;
        while mul <= max {
            mul *= 2;
        }

        let nums = nums
            .into_iter()
            .map(|num| num as u128 * mul as u128)
            .collect::<Vec<_>>();
        let sum = sum * mul as u128;

        let mut reduced = 0;
        let mut count = 0;
        let mut heap = BinaryHeap::from(nums);

        while 2 * reduced < sum {
            let num = heap.pop().unwrap() / 2;
            reduced += num;
            heap.push(num);
            count += 1;
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2208() {
        assert_eq!(Solution::halve_array(vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(Solution::halve_array(vec![5, 19, 8, 1]), 3);
        assert_eq!(Solution::halve_array(vec![3, 8, 20]), 3);
    }
}
