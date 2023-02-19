pub struct Solution {}

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let mut acc = vec![0; chalk.len()];
        acc[0] = chalk[0] as usize;
        for i in 1..acc.len() {
            acc[i] = acc[i - 1] + chalk[i] as usize;
        }

        let remains = k as usize % acc.last().unwrap();
        match acc.binary_search(&remains) {
            Ok(i) => i as i32 + 1,
            Err(i) => i as i32,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1894() {
        assert_eq!(Solution::chalk_replacer(vec![5, 1, 5], 22), 0);
        assert_eq!(Solution::chalk_replacer(vec![3, 4, 1, 2], 25), 1);
    }
}
