pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut distances = vec![vec![0; points.len()]; points.len()];
        for i in 0..points.len() {
            for j in i..points.len() {
                if i != j {
                    let d =
                        (points[i][0] - points[j][0]).pow(2) + (points[i][1] - points[j][1]).pow(2);
                    distances[i][j] = d;
                    distances[j][i] = d;
                }
            }
        }
        let mut result = 0;
        for i in 0..points.len() {
            let mut counts = HashMap::new();
            for j in 0..points.len() {
                if i != j {
                    (*counts.entry(distances[i][j]).or_insert(0)) += 1;
                }
            }
            for c in counts.values() {
                result += c * (c - 1);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0447() {
        assert_eq!(
            8,
            Solution::number_of_boomerangs(vec![vec![0, 0], vec![0, 3], vec![3, 0], vec![0, -3]])
        );
        assert_eq!(
            2,
            Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]])
        );
        assert_eq!(
            2,
            Solution::number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]])
        );
        assert_eq!(0, Solution::number_of_boomerangs(vec![vec![1, 1]]));
    }
}
