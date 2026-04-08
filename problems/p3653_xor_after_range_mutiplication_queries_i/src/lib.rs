pub struct Solution {}

impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const M: i64 = 1_000_000_007;
        let mut mul = vec![1; nums.len()];
        for i in 0..queries.len() {
            for j in
                (queries[i][0] as usize..=queries[i][1] as usize).step_by(queries[i][2] as usize)
            {
                mul[j] *= queries[i][3] as i64;
                mul[j] %= M;
            }
        }

        let mut result = 0;
        for i in 0..nums.len() {
            result ^= nums[i] as i64 * mul[i] % M;
        }

        println!("mul: {:?}", mul);

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3653() {
        assert_eq!(
            Solution::xor_after_queries(vec![1, 1, 1], vec![vec![0, 2, 1, 4]]),
            4
        );
        assert_eq!(
            Solution::xor_after_queries(
                vec![2, 3, 1, 5, 4],
                vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]]
            ),
            31
        );
    }
}
