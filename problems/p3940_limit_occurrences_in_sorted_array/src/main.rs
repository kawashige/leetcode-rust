pub struct Solution {}

impl Solution {
    pub fn limit_occurrences(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = [0; 101];
        let mut result = Vec::new();
        for n in nums {
            if count[n as usize] != k {
                count[n as usize] += 1;
                result.push(n);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3940() {
        assert_eq!(
            Solution::limit_occurrences(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 1, 2, 2, 3]
        );
        assert_eq!(Solution::limit_occurrences(vec![1, 2, 3], 1), vec![1, 2, 3]);
    }
}

fn main() {}
