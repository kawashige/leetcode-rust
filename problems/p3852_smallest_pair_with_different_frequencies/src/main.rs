pub struct Solution {}

impl Solution {
    pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = [0; 101];
        for i in 0..nums.len() {
            freq[nums[i] as usize] += 1;
        }

        for x in 1..freq.len() {
            if freq[x] == 0 {
                continue;
            }
            for y in x + 1..freq.len() {
                if freq[y] != 0 && freq[x] != freq[y] {
                    return vec![x as i32, y as i32];
                }
            }
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3582() {
        assert_eq!(
            Solution::min_distinct_freq_pair(vec![1, 1, 2, 2, 3, 4]),
            vec![1, 3]
        );
        assert_eq!(Solution::min_distinct_freq_pair(vec![1, 5]), vec![-1, -1]);
        assert_eq!(Solution::min_distinct_freq_pair(vec![7]), vec![-1, -1]);
    }
}

fn main() {}
