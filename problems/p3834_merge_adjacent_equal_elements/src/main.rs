pub struct Solution {}

impl Solution {
    pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
        let mut result = Vec::new();
        for v in nums {
            result.push(v as i64);
            while 1 < result.len() && result[result.len() - 1] == result[result.len() - 2] {
                result.pop();
                *result.last_mut().unwrap() *= 2;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3834() {
        assert_eq!(Solution::merge_adjacent(vec![3, 1, 1, 2]), vec![3, 4]);
        assert_eq!(Solution::merge_adjacent(vec![2, 2, 4]), vec![8]);
        assert_eq!(Solution::merge_adjacent(vec![3, 7, 5]), vec![3, 7, 5]);
    }
}

fn main() {}
