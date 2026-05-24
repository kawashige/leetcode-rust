pub struct Solution {}

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;

        let mut val_with_index = (0..arr.len()).collect::<Vec<_>>();
        val_with_index.sort_unstable_by_key(|i| -arr[*i]);

        let mut dp = vec![1; arr.len()];

        for i in val_with_index {
            for j in (i - d.min(i)..i).rev() {
                if arr[i] <= arr[j] {
                    break;
                }
                dp[j] = dp[j].max(dp[i] + 1);
            }
            for j in i + 1..(i + 1 + d).min(arr.len()) {
                if arr[i] <= arr[j] {
                    break;
                }
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }

        dp.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1340() {
        assert_eq!(
            Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2),
            4
        );
        assert_eq!(Solution::max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
        assert_eq!(Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
    }
}

fn main() {}
