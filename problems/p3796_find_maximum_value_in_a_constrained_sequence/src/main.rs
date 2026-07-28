pub struct Solution {}

impl Solution {
    pub fn find_max_val(n: i32, restrictions: Vec<Vec<i32>>, diff: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut restrictions2 = vec![std::i32::MAX; n];
        for r in restrictions {
            restrictions2[r[0] as usize] = r[1];
        }

        let mut max_values = vec![0; n];

        for i in 1..n {
            max_values[i] += max_values[i - 1] + diff[i - 1];
            max_values[i] = max_values[i].min(restrictions2[i]);
        }
        for i in (0..n - 1).rev() {
            max_values[i] = max_values[i].min(max_values[i + 1] + diff[i]);
        }

        max_values.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3796() {
        assert_eq!(
            Solution::find_max_val(
                10,
                vec![vec![3, 1], vec![8, 1]],
                vec![2, 2, 3, 1, 4, 5, 1, 1, 2]
            ),
            6
        );
        assert_eq!(
            Solution::find_max_val(8, vec![vec![3, 2]], vec![3, 5, 2, 4, 2, 3, 1]),
            12
        );
    }
}

fn main() {}
