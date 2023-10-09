pub struct Solution {}

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut count = vec![0; 1001];

        for i in 1..nums.len() {
            if nums[i - 1] == key {
                count[nums[i] as usize] += 1;
            }
        }

        (0..count.len()).max_by_key(|i| count[*i]).unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2190() {
        assert_eq!(Solution::most_frequent(vec![1, 100, 200, 1, 100], 1), 100);
        assert_eq!(Solution::most_frequent(vec![2, 2, 2, 2, 3], 2), 2);
    }
}
