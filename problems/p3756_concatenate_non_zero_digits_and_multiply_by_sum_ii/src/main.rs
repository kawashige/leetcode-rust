pub struct Solution {}

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const M: usize = 1_000_000_007;
        let mut acc = vec![0; s.len() + 1];
        let mut acc_x = vec![0; s.len() + 1];
        let mut x = 0;
        let mut len_x = vec![0; s.len() + 1];
        let mut len = 0;
        for i in 0..s.len() {
            let d = (s.as_bytes()[i] - b'0') as usize;
            acc[i + 1] = (d + acc[i]) % M;
            if d != 0 {
                x = (x * 10 + d) % M;
                len += 1;
            }
            acc_x[i + 1] = x;
            len_x[i + 1] = len;
        }

        let mut pow = vec![1; len + 1];
        for i in 1..=len {
            pow[i] = (pow[i - 1] * 10) % M;
        }

        let mut result = Vec::new();
        for q in queries {
            let sum = (acc[q[1] as usize + 1] - acc[q[0] as usize]) % M;
            let l = (len_x[q[1] as usize + 1] - len_x[q[0] as usize]) as usize;
            let x = ((acc_x[q[1] as usize + 1] - acc_x[q[0] as usize] * pow[l] % M) + M) % M;
            result.push(((sum * x) % M) as i32);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3756() {
        assert_eq!(
            Solution::sum_and_multiply(
                "10203004".to_string(),
                vec![vec![0, 7], vec![1, 3], vec![4, 6]]
            ),
            vec![12340, 4, 9]
        );
        assert_eq!(
            Solution::sum_and_multiply("1000".to_string(), vec![vec![0, 3], vec![1, 1]]),
            vec![1, 0]
        );
        assert_eq!(
            Solution::sum_and_multiply("9876543210".to_string(), vec![vec![0, 9]]),
            vec![444444137]
        );
    }
}

fn main() {}
