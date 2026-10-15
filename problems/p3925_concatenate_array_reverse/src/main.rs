pub struct Solution {}

impl Solution {
    pub fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().chain(nums.iter().rev()).cloned().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3925() {
        assert_eq!(
            Solution::concat_with_reverse(vec![1, 2, 3]),
            vec![1, 2, 3, 3, 2, 1]
        );
        assert_eq!(Solution::concat_with_reverse(vec![1]), vec![1]);
    }
}

fn main() {}
