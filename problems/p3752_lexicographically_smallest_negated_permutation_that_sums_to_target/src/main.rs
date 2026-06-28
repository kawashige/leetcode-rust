pub struct Solution {}

impl Solution {
    pub fn lex_smallest_negated_perm(n: i32, target: i64) -> Vec<i32> {
        let sum = (n as i64 * (n as i64 + 1)) / 2;
        if sum == target {
            return (1..=n).collect();
        } else if sum < target {
            return Vec::new();
        }

        let mut remains = sum - target;
        let mut val = (remains / 2).min(n as i64);
        let mut result = Vec::new();
        let mut used = vec![false; n as usize + 1];

        while 0 < remains && 0 < val {
            result.push(-val as i32);
            remains -= val * 2;
            used[val as usize] = true;
            val = (val - 1).min(remains / 2);
        }

        if remains != 0 {
            return Vec::new();
        }
        for i in 1..=n as usize {
            if !used[i] {
                result.push(i as i32);
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3752() {
        assert_eq!(Solution::lex_smallest_negated_perm(3, 0), vec![-3, 1, 2]);
        assert_eq!(Solution::lex_smallest_negated_perm(1, 10000000000), vec![]);
    }
}

fn main() {}
