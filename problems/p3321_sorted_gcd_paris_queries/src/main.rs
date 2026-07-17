pub struct Solution {}

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let m = *nums.iter().max().unwrap() as usize;
        let mut cnt = vec![0i64; m + 1];
        for &num in &nums {
            cnt[num as usize] += 1;
        }
        for i in 1..=m {
            let mut j = i * 2;
            while j <= m {
                cnt[i] += cnt[j];
                j += i;
            }
        }
        for i in 1..=m {
            cnt[i] = cnt[i] * (cnt[i] - 1) / 2;
        }
        for i in (1..=m).rev() {
            let mut j = i * 2;
            while j <= m {
                cnt[i] -= cnt[j];
                j += i;
            }
        }
        for i in 1..=m {
            cnt[i] += cnt[i - 1];
        }
        let mut ans = Vec::new();
        for &q in &queries {
            let q = (q + 1) as i64;
            let pos = cnt.partition_point(|&x| x < q);
            ans.push(pos as i32);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3321() {
        assert_eq!(
            Solution::gcd_values(vec![2, 3, 4], vec![0, 2, 2]),
            vec![1, 2, 2]
        );
        assert_eq!(
            Solution::gcd_values(vec![4, 4, 2, 1], vec![5, 3, 1, 0]),
            vec![4, 2, 1, 1]
        );
        assert_eq!(Solution::gcd_values(vec![2, 2], vec![0, 0]), vec![2, 2]);
    }
}

fn main() {}
