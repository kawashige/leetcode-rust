pub struct Solution {}

impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut rotated = vec![vec!['.'; b.len()]; b[0].len()];

        for i in 0..b.len() {
            let mut count = 0;
            for j in 0..b[0].len() {
                if b[i][j] == '#' {
                    count += 1;
                } else if b[i][j] == '*' {
                    rotated[j][b.len() - 1 - i] = '*';
                }

                if b[i][j] == '*' || j == b[0].len() - 1 {
                    // println!("count: {}", count);
                    for k in 0..count {
                        rotated[j - k - if b[i][j] == '*' { 1 } else { 0 }][b.len() - 1 - i] = '#';
                    }
                    count = 0;
                }
            }
        }

        rotated
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1861() {
        assert_eq!(
            Solution::rotate_the_box(vec![vec!['#', '.', '#']]),
            vec![vec!['.'], vec!['#'], vec!['#']]
        );
        assert_eq!(
            Solution::rotate_the_box(vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']]),
            vec![
                vec!['#', '.'],
                vec!['#', '#'],
                vec!['*', '*'],
                vec!['.', '.']
            ]
        );
        assert_eq!(
            Solution::rotate_the_box(vec![
                vec!['#', '#', '*', '.', '*', '.'],
                vec!['#', '#', '#', '*', '.', '.'],
                vec!['#', '#', '#', '.', '#', '.']
            ]),
            vec![
                vec!['.', '#', '#'],
                vec!['.', '#', '#'],
                vec!['#', '#', '*'],
                vec!['#', '*', '.'],
                vec!['#', '.', '*'],
                vec!['#', '.', '.']
            ]
        );
    }
}
