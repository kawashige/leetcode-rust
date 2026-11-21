pub struct Solution {}

impl Solution {
    pub fn internal_angles(sides: Vec<i32>) -> Vec<f64> {
        let mut sides = sides;
        sides.sort_unstable();

        if sides[0] + sides[1] <= sides[2] {
            return Default::default();
        }

        let sides = sides.into_iter().map(|s| s as f64).collect::<Vec<_>>();
        let square = sides.iter().map(|s| s * s).collect::<Vec<_>>();
        let cos_a = (square[1] + square[2] - square[0]) / (2.0 * sides[1] * sides[2]);
        let cos_b = (square[0] + square[2] - square[1]) / (2.0 * sides[0] * sides[2]);
        let cos_c = (square[0] + square[1] - square[2]) / (2.0 * sides[0] * sides[1]);

        let mut result = vec![
            cos_a.acos().to_degrees(),
            cos_b.acos().to_degrees(),
            cos_c.acos().to_degrees(),
        ];
        result.sort_by(|a, b| a.total_cmp(b));
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3899() {
        assert_eq!(
            Solution::internal_angles(vec![3, 4, 5]),
            vec![36.86990, 53.13010, 90.00000]
        );
        assert_eq!(Solution::internal_angles(vec![2, 4, 2]), vec![]);
    }
}

fn main() {}
