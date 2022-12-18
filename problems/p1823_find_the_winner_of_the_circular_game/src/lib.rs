pub struct Solution {}

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut deleted = vec![false; n as usize];
        let mut i = 0;
        for _ in 0..n - 1 {
            let mut remains = k;
            while 1 < remains {
                i = (i + 1) % deleted.len();
                while deleted[i] {
                    i = (i + 1) % deleted.len();
                }
                remains -= 1;
            }
            deleted[i] = true;
            i = (i + 1) % deleted.len();
            while deleted[i] {
                i = (i + 1) % deleted.len();
            }
        }
        (0..deleted.len()).find(|i| !deleted[*i]).unwrap() as i32 + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1823() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
        assert_eq!(Solution::find_the_winner(6, 5), 1);
    }
}
