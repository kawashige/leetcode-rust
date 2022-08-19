pub struct Solution {}

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let count = nums.into_iter().fold([0; 1001], |mut count, x| {
            count[x as usize] += 1;
            count
        });

        let mut sum = 0;
        for i in (0..count.len()).rev() {
            sum += count[i];
            if sum == i {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1608() {
        assert_eq!(Solution::special_array(vec![3, 5]), 2);
        assert_eq!(Solution::special_array(vec![0, 0]), -1);
        assert_eq!(Solution::special_array(vec![0, 4, 3, 0, 4]), 3);
    }
}
