pub struct Solution {}

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        (0..variables.len() as i32)
            .filter(|i| {
                let v = &variables[*i as usize];
                let mut x = v[0] % 10;
                for _ in 1..v[1] {
                    x *= v[0] % 10;
                    x %= 10;
                }
                x %= v[3];
                let y = x;
                for _ in 1..v[2] {
                    x *= y;
                    x %= v[3];
                }
                x == target
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2961() {
        assert_eq!(
            Solution::get_good_indices(
                vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]],
                2
            ),
            vec![0, 2]
        );
        assert_eq!(
            Solution::get_good_indices(vec![vec![39, 3, 1000, 1000]], 17),
            vec![]
        );
    }
}
