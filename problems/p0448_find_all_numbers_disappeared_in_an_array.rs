pub struct Solution {}

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            if nums[i] as usize == i + 1 {
                continue;
            }
            let mut n = nums[i];
            nums[i] = 0;
            while n != 0 && nums[n as usize - 1] != n {
                let tmp = nums[n as usize - 1];
                nums[n as usize - 1] = n;
                n = tmp;
            }
        }

        nums.into_iter()
            .zip(1..)
            .filter(|(i, _)| i == &0)
            .map(|(_, j)| j)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0448() {
        assert_eq!(
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
        assert_eq!(
            vec![] as Vec<i32>,
            Solution::find_disappeared_numbers(vec![1])
        );
    }
}
