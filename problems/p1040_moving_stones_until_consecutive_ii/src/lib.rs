pub struct Solution {}

impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let mut stones = stones;
        stones.sort_unstable();

        let mut min = std::i32::MAX;
        let mut l = 0;
        for i in 0..stones.len() {
            while (stones.len() as i32) < stones[i] - stones[l] + 1 {
                l += 1;
            }
            if i - l + 1 == stones.len() - 1 && stones[i] - stones[l] + 1 == stones.len() as i32 - 1
            {
                min = min.min(2)
            } else {
                min = min.min((stones.len() - (i - l + 1)) as i32);
            }
        }

        let max = (stones[stones.len() - 2] - stones[0] + 2)
            .max(stones[stones.len() - 1] - stones[1] + 2)
            - stones.len() as i32;

        vec![min, max]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1040() {
        assert_eq!(Solution::num_moves_stones_ii(vec![7, 4, 9]), vec![1, 2]);
        assert_eq!(
            Solution::num_moves_stones_ii(vec![6, 5, 4, 3, 10]),
            vec![2, 3]
        );
    }
}
