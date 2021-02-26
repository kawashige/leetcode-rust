use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn knight_probability(n: i32, k: i32, r: i32, c: i32) -> f64 {
        fn recurse(
            n: i32,
            k: i32,
            r: i32,
            c: i32,
            memo: &mut HashMap<(i32, i32, i32), f64>,
            org_k: i32,
        ) -> f64 {
            if r < 0 || c < 0 || n - 1 < r || n - 1 < c {
                return 0.0;
            }
            if k == 0 {
                return 1.0;
            }
            if let Some(v) = memo.get(&(k, r, c)) {
                return *v;
            }

            let mut result = 0.0;
            for (m_r, m_c) in [
                (-2, -1),
                (-2, 1),
                (-1, -2),
                (-1, 2),
                (1, -2),
                (1, 2),
                (2, -1),
                (2, 1),
            ]
            .iter()
            {
                result += recurse(n, k - 1, r + m_r, c + m_c, memo, org_k) / 8.0;
            }

            memo.insert((k, r, c), result);
            result
        }

        recurse(n, k, r, c, &mut HashMap::new(), k)
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0688() {
        assert_eq!(Solution::knight_probability(20, 83, 7, 13), 0.01517);
        assert_eq!(Solution::knight_probability(3, 2, 0, 0), 0.0625);
    }
}
