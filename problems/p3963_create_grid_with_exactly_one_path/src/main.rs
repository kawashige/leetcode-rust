pub struct Solution {}

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut result = Vec::new();
        for _ in 0..m - 1 {
            result.push(
                std::iter::once('.')
                    .chain(std::iter::repeat('#').take(n as usize - 1))
                    .collect(),
            );
        }
        result.push(std::iter::repeat('.').take(n as usize).collect());
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3963() {
        assert_eq!(
            Solution::create_grid(2, 3),
            vec!["..#".to_string(), "#..".to_string()]
        );
        assert_eq!(
            Solution::create_grid(3, 3),
            vec!["..#".to_string(), "#..".to_string(), "##.".to_string()]
        );
        assert_eq!(Solution::create_grid(1, 4), vec!["....".to_string()]);
    }
}

fn main() {}
