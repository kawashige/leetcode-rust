pub struct Solution {}

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut values = vec![vec![]; 3];
        for n in nums {
            values[(n as usize) % 3].push(n);
        }
        for i in 0..values.len() {
            values[i].sort_unstable_by(|a, b| b.cmp(&a));
        }

        let mut result = 0;
        if values.iter().all(|v| !v.is_empty()) {
            result = values[0][0] + values[1][0] + values[2][0];
        }
        for i in 0..values.len() {
            if 2 < values[i].len() {
                result = result.max(values[i][..3].iter().sum());
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3780() {
        assert_eq!(Solution::maximum_sum(vec![4, 2, 3, 1]), 9);
        assert_eq!(Solution::maximum_sum(vec![2, 1, 5]), 0);
    }
}

fn main() {}
