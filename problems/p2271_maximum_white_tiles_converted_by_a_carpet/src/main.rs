pub struct Solution {}

impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut tiles = tiles;
        tiles.sort_unstable();

        let mut j = 0;
        let mut count = 0;
        let mut result = 0;

        for i in 0..tiles.len() {
            while j + 1 < tiles.len() && tiles[j + 1][0] <= tiles[i][0] + carpet_len - 1 {
                count += tiles[j][1] - tiles[j][0] + 1;
                j += 1;
            }
            result = result
                .max(count + (tiles[i][0] + carpet_len - 1).min(tiles[j][1]) - tiles[j][0] + 1);
            count -= tiles[i][1] - tiles[i][0] + 1;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2271() {
        assert_eq!(
            Solution::maximum_white_tiles(
                vec![
                    vec![1, 5],
                    vec![10, 11],
                    vec![12, 18],
                    vec![20, 25],
                    vec![30, 32]
                ],
                10
            ),
            9
        );
        assert_eq!(
            Solution::maximum_white_tiles(vec![vec![10, 11], vec![1, 1]], 2),
            2
        );
    }
}
