pub struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn recurse(image: &mut Vec<Vec<i32>>, sr: usize, sc: usize, new_color: i32) {
            let org_color = image[sr][sc];
            if org_color == new_color {
                return;
            }

            image[sr][sc] = new_color;

            if 0 < sr && image[sr - 1][sc] == org_color {
                recurse(image, sr - 1, sc, new_color);
            }
            if 0 < sc && image[sr][sc - 1] == org_color {
                recurse(image, sr, sc - 1, new_color);
            }
            if sr < image.len() - 1 && image[sr + 1][sc] == org_color {
                recurse(image, sr + 1, sc, new_color);
            }
            if sc < image[0].len() - 1 && image[sr][sc + 1] == org_color {
                recurse(image, sr, sc + 1, new_color);
            }
        }

        let mut image = image;
        recurse(&mut image, sr as usize, sc as usize, new_color);
        image
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0733() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        )
    }
}
