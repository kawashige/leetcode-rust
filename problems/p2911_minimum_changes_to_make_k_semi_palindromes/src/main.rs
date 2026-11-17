pub struct Solution {}

fn segment_cost(s: &[u8], l: usize, m: usize, factors: &Vec<Vec<usize>>) -> i32 {
    let mut best = m as i32;
    for &d in &factors[m] {
        let groups = m / d; // Each group contains groups characters
        let mut changes = 0;
        for offset in 0..d {
            for t in 0..groups / 2 {
                // Pair characters from opposite ends of the group
                if s[l + offset + t * d] != s[l + offset + (groups - 1 - t) * d] {
                    changes += 1;
                }
            }
        }
        best = best.min(changes);
    }
    best
}

fn solve(
    seg: usize,
    start: usize,
    n: usize,
    cost: &Vec<Vec<i32>>,
    memo: &mut Vec<Vec<i32>>,
) -> i32 {
    if seg == 1 {
        // Only one segment remains
        return cost[start][n - 1];
    }
    if memo[seg][start] != -1 {
        // Already computed
        return memo[seg][start];
    }
    let mut best = n as i32;
    let limit = n - 1 - 2 * (seg - 1); // Leave room for seg-1 segments after the first segment
    for e in (start + 1)..=limit {
        best = best.min(cost[start][e] + solve(seg - 1, e + 1, n, cost, memo));
    }
    memo[seg][start] = best;
    best
}

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        let mut factors = vec![vec![]; n + 1];
        // Precompute the proper divisors of each number
        for d in 1..=n / 2 {
            let mut m = d * 2;
            while m <= n {
                factors[m].push(d);
                m += d;
            }
        }
        let mut cost = vec![vec![0i32; n]; n];
        for l in 0..n - 1 {
            for r in (l + 1)..n {
                // Each substring must have length at least 2
                cost[l][r] = segment_cost(s, l, r - l + 1, &factors);
            }
        }
        let mut memo = vec![vec![-1i32; n]; k + 1];
        solve(k, 0, n, &cost, &mut memo)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2911() {
        assert_eq!(Solution::minimum_changes("abcac".to_string(), 2), 1);
        assert_eq!(Solution::minimum_changes("abcdef".to_string(), 2), 2);
        assert_eq!(Solution::minimum_changes("aabbaa".to_string(), 3), 0);
    }
}

fn main() {}
