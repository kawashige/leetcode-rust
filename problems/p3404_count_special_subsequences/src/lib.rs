pub struct Solution {}

impl Solution {
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            return n;
        } else {
            Self::gcd(n % m, m)
        }
    }

    pub fn number_of_subsequences(nums: Vec<i32>) -> i64 {
        let mut count: Vec<Vec<usize>> = vec![vec![0; 1001]; 1001];
        let mut tmp: Vec<Vec<(usize, usize)>> = vec![vec![]; nums.len() + 2];
        let mut result = 0;

        for q in (2..nums.len()).rev() {
            for (v_s, v_r) in &tmp[q + 2] {
                count[*v_s][*v_r] += 1;
            }

            for p in 0..q - 1 {
                let mut v_p = nums[p];
                let mut v_q = nums[q];
                loop {
                    let x = Self::gcd(v_p, v_q);
                    if x == 1 {
                        break;
                    }
                    v_p /= x;
                    v_q /= x;
                }

                result += count[v_q as usize][v_p as usize];
                tmp[p].push((v_p as usize, v_q as usize));
            }
        }

        result as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3404() {
        assert_eq!(
            Solution::number_of_subsequences(vec![
                16, 6, 4, 3, 17, 12, 9, 4, 48, 17, 10, 18, 40, 68, 14, 40, 14, 20, 7
            ]),
            22
        );
        assert_eq!(
            Solution::number_of_subsequences(vec![1, 2, 3, 4, 3, 6, 1]),
            1
        );
        assert_eq!(
            Solution::number_of_subsequences(vec![3, 4, 3, 4, 3, 4, 3, 4]),
            3
        );
    }
}
