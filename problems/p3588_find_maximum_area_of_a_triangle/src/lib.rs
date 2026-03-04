pub struct Solution {}

impl Solution {
    pub fn find(coords: Vec<Vec<i32>>) -> i64 {
        let mut i = 0;
        let mut result = -1;

        while i < coords.len() {
            let mut j = i;
            while j + 1 < coords.len() && coords[i][0] == coords[j + 1][0] {
                j += 1;
            }
            if j == i {
                i += 1;
                continue;
            }

            if coords[coords.len() - 1][0] != coords[i][0] {
                result = result.max(
                    (coords[j][1] - coords[i][1]) as i64
                        * (coords[coords.len() - 1][0] - coords[i][0]) as i64,
                );
            }
            if coords[0][0] != coords[i][0] {
                result = result.max(
                    (coords[j][1] - coords[i][1]) as i64 * (coords[i][0] - coords[0][0]) as i64,
                );
            }

            i = j + 1;
        }

        result
    }
    pub fn max_area(coords: Vec<Vec<i32>>) -> i64 {
        let mut coords1 = coords.clone();
        let mut coords2 = coords
            .into_iter()
            .map(|coord| vec![coord[1], coord[0]])
            .collect::<Vec<_>>();
        coords1.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));
        coords2.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(a[1].cmp(&b[1])));

        Self::find(coords1).max(Self::find(coords2))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3588() {
        assert_eq!(
            Solution::max_area(vec![vec![1, 1], vec![1, 2], vec![3, 2], vec![3, 3]]),
            2
        );
        assert_eq!(
            Solution::max_area(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            -1
        );
    }
}
