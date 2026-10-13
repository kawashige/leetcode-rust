pub struct Solution {}

impl Solution {
    pub fn count_opposite_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut count = [0; 2];

        for i in (0..nums.len()).rev() {
            result[i] = count[((nums[i] % 2 + 1) % 2) as usize];
            count[(nums[i] % 2) as usize] += 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3917() {
        assert_eq!(
            Solution::count_opposite_parity(vec![1, 2, 3, 4]),
            vec![2, 1, 1, 0]
        );
        assert_eq!(Solution::count_opposite_parity(vec![1]), vec![0]);
    }
}

fn main() {}
