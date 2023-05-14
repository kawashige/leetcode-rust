use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn recurse(unused: usize, gcds: &Vec<Vec<i32>>, memo: &mut HashMap<usize, i32>) -> i32 {
        if unused == 0 {
            return 0;
        }
        if let Some(max_score) = memo.get(&unused) {
            return *max_score;
        }

        let mut max_score = 0;
        for i in 0..gcds.len() {
            if unused & 1 << i == 0 {
                continue;
            }
            for j in (i + 1)..gcds.len() {
                if unused & 1 << j == 0 {
                    continue;
                }
                max_score = max_score.max(
                    gcds[i][j] * unused.count_ones() as i32 / 2
                        + Self::recurse(unused ^ (1 << i | 1 << j), gcds, memo),
                );
            }
        }

        memo.insert(unused, max_score);
        max_score
    }

    pub fn max_score(nums: Vec<i32>) -> i32 {
        let mut gcds = vec![vec![0; nums.len()]; nums.len()];
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let x = Self::gcd(nums[i], nums[j]);
                gcds[i][j] = x;
                gcds[j][i] = x;
            }
        }

        Self::recurse(
            2_usize.pow(nums.len() as u32) - 1,
            &gcds,
            &mut HashMap::new(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1799() {
        // assert_eq!(Solution::max_score(vec![1, 2]), 1);
        // assert_eq!(Solution::max_score(vec![3, 4, 6, 8]), 11);
        assert_eq!(Solution::max_score(vec![1, 2, 3, 4, 5, 6]), 14);
    }
}
