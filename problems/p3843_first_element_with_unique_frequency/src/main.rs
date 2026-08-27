pub struct Solution {}

impl Solution {
    pub fn first_unique_freq(nums: Vec<i32>) -> i32 {
        let mut count = vec![0; 100_001];
        for &n in &nums {
            count[n as usize] += 1;
        }
        let mut freq_count = vec![0; 100_001];
        for &c in &count {
            freq_count[c] += 1;
        }

        nums.into_iter()
            .find(|n| freq_count[count[*n as usize]] == 1)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3843() {
        assert_eq!(Solution::first_unique_freq(vec![20, 10, 30, 30]), 30);
        assert_eq!(
            Solution::first_unique_freq(vec![20, 20, 10, 30, 30, 30]),
            20
        );
        assert_eq!(Solution::first_unique_freq(vec![10, 10, 20, 20]), -1);
    }
}

fn main() {}
