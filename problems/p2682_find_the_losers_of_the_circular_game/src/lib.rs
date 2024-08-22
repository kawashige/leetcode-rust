pub struct Solution {}

impl Solution {
    pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
        let mut received = vec![false; n as usize];
        let mut cur = 0;
        received[0] = true;
        let k = k as usize;

        for i in 1.. {
            cur = (cur + i * k) % received.len();
            if received[cur] {
                break;
            }
            received[cur] = true;
        }

        (0..received.len())
            .filter(|i| !received[*i])
            .map(|i| i as i32 + 1)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2682() {
        assert_eq!(Solution::circular_game_losers(5, 2), vec![4, 5]);
        assert_eq!(Solution::circular_game_losers(4, 4), vec![2, 3, 4]);
    }
}
