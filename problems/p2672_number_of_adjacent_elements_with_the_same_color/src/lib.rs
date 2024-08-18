pub struct Solution {}

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut colors = vec![0; n as usize];
        let mut pairs = 0;
        let mut result = Vec::with_capacity(queries.len());

        for q in queries {
            if 0 < q[0]
                && colors[q[0] as usize] != 0
                && colors[q[0] as usize - 1] == colors[q[0] as usize]
            {
                pairs -= 1;
            }
            if q[0] < n - 1
                && colors[q[0] as usize] != 0
                && colors[q[0] as usize] == colors[q[0] as usize + 1]
            {
                pairs -= 1;
            }
            colors[q[0] as usize] = q[1];
            if 0 < q[0]
                && colors[q[0] as usize] != 0
                && colors[q[0] as usize - 1] == colors[q[0] as usize]
            {
                pairs += 1;
            }
            if q[0] < n - 1
                && colors[q[0] as usize] != 0
                && colors[q[0] as usize] == colors[q[0] as usize + 1]
            {
                pairs += 1;
            }
            result.push(pairs);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2672() {
        assert_eq!(
            Solution::color_the_array(
                4,
                vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]]
            ),
            vec![0, 1, 1, 0, 2]
        );
        assert_eq!(Solution::color_the_array(1, vec![vec![0, 100000]]), vec![0]);
    }
}
