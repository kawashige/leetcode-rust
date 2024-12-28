pub struct Solution {}

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut max = vec![vec![(0, vec![]); 4]; nums.len()];
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i] as usize;
            if k <= i as i32 {
                sum -= nums[i - k as usize] as usize;
            }
            if 0 < i {
                max[i][1] = max[i - 1][1].clone();
                max[i][2] = max[i - 1][2].clone();
                max[i][3] = max[i - 1][3].clone();
            }
            if k - 1 <= i as i32 && max[i][1].0 < sum {
                max[i][1] = (sum, vec![i as i32 + 1 - k]);
            }

            if (k as usize) <= i
                && 0 < max[i - k as usize][1].0
                && max[i][2].0 < max[i - k as usize][1].0 + sum
            {
                max[i][2] = (
                    max[i - k as usize][1].0 + sum,
                    vec![max[i - k as usize][1].1[0], i as i32 + 1 - k],
                );
            }
            if (k as usize) <= i
                && 0 < max[i - k as usize][2].0
                && max[i][3].0 < max[i - k as usize][2].0 + sum
            {
                max[i][3] = (
                    max[i - k as usize][2].0 + sum,
                    vec![
                        max[i - k as usize][2].1[0],
                        max[i - k as usize][2].1[1],
                        i as i32 + 1 - k,
                    ],
                );
            }
            println!("{}: {:?}", i, max[i]);
        }
        max[nums.len() - 1][3].1.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0689() {
        assert_eq!(
            Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
            vec![0, 3, 5]
        );
        assert_eq!(
            Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
            vec![0, 2, 4]
        );
    }
}
