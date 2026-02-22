pub struct Solution {}

impl Solution {
    const M: usize = 1_000_000_007;

    pub fn modinv(a: usize) -> usize {
        let mut a = a as i64;
        let m = 1_000_000_007;
        let mut b = m;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u %= m;
        if u < 0 {
            u += m;
        }
        u as usize
    }

    pub fn combination(n: usize, k: usize, factorial: &[usize]) -> usize {
        ((factorial[n] * Self::modinv(factorial[k])) % Self::M) * Self::modinv(factorial[n - k])
            % Self::M
    }

    pub fn find_max_depth(cur: usize, prev: usize, list: &Vec<Vec<usize>>, d: usize) -> usize {
        let mut result = d;
        for next in &list[cur] {
            if &prev == next {
                continue;
            }
            result = result.max(Self::find_max_depth(*next, cur, list, d + 1));
        }
        result
    }
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let mut list = vec![vec![]; edges.len() + 1];
        for e in edges {
            list[e[0] as usize - 1].push(e[1] as usize - 1);
            list[e[1] as usize - 1].push(e[0] as usize - 1);
        }
        let max_depth = Self::find_max_depth(0, list.len(), &list, 0);

        let mut factorial = vec![1; max_depth + 1];
        for i in 1..factorial.len() {
            factorial[i] *= factorial[i - 1] * i;
            factorial[i] %= Self::M;
        }

        let mut result = max_depth;
        for i in (3..=max_depth).step_by(2) {
            result += Self::combination(max_depth, i, &factorial);
            result %= Self::M;
        }
        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3558() {
        assert_eq!(Solution::assign_edge_weights(vec![vec![1, 2]]), 1);
        assert_eq!(
            Solution::assign_edge_weights(vec![vec![1, 2], vec![1, 3], vec![3, 4], vec![3, 5]]),
            2
        );
    }
}
