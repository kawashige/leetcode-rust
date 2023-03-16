pub struct Solution {}

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut exists = vec![false; (n * n + 1) as usize];
        for i in 1..=n as usize {
            exists[i * i] = true
        }

        let mut count = 0;

        for i in 1..=n as usize {
            for j in i + 1..=n as usize {
                if exists.len() <= i * i + j * j {
                    break;
                }
                if exists[i * i + j * j] {
                    count += 2;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1925() {
        assert_eq!(Solution::count_triples(5), 2);
        assert_eq!(Solution::count_triples(10), 4);
    }
}
