pub struct Solution {}

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut pre = vec![0; n];
        pre[0] = stones[0];
        for i in 1..n {
            pre[i] = pre[i - 1] + stones[i];
        }

        let mut f = vec![0; n];
        f[n - 1] = pre[n - 1];
        for i in (1..n - 1).rev() {
            f[i] = f[i + 1].max(pre[i] - f[i + 1]);
        }
        f[1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1872() {
        assert_eq!(Solution::stone_game_viii(vec![-1, 2, -3, 4, -5]), 5);
        assert_eq!(Solution::stone_game_viii(vec![7, -6, 5, 10, 5, -2, -6]), 13);
        assert_eq!(Solution::stone_game_viii(vec![-10, -12]), -22);
    }
}

fn main() {}
