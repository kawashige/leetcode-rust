pub struct Solution {}

impl Solution {
    pub fn dfs(
        start: usize,
        divs: &[i32],
        prod: i32,
        path: &mut Vec<i32>,
        best_path: &mut Vec<i32>,
        k: usize,
        n: i32,
    ) {
        if k < path.len() {
            return;
        }
        if prod == n {
            if path.len() == k || path[0] == 1 {
                if path[path.len() - 1] - path[0] < best_path[best_path.len() - 1] {
                    *best_path = path.clone();
                }
            }
            return;
        }

        for i in start..divs.len() {
            if n < prod * divs[i] {
                break;
            }
            (*path).push(divs[i]);
            Self::dfs(i + 1, divs, prod * divs[i], path, best_path, k, n);
            (*path).pop();
        }
    }

    pub fn min_difference(n: i32, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut divs = Vec::new();
        divs.push(1);

        for i in 2..=n {
            if n % i == 0 {
                let mut x = n;
                while x % i == 0 {
                    divs.push(i);
                    x /= i;
                }
            }
        }

        let mut best_path = vec![1; k];
        *best_path.last_mut().unwrap() = n;

        Self::dfs(0, &divs, 1, &mut Vec::new(), &mut best_path, k, n);

        if best_path.len() < k {
            std::iter::repeat(1)
                .take(k - best_path.len())
                .chain(best_path.into_iter())
                .collect()
        } else {
            best_path
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3669() {
        assert_eq!(Solution::min_difference(100, 2), vec![10, 10]);
        assert_eq!(Solution::min_difference(44, 3), vec![2, 2, 11]);
    }
}
