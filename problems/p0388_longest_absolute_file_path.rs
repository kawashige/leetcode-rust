pub struct Solution {}

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut result = 0;

        let mut dirs = Vec::new();
        for s in input.split('\n') {
            let depth = s.chars().take_while(|c| c == &'\t').count();
            while depth < dirs.len() {
                dirs.pop();
            }
            if s.contains('.') {
                result = std::cmp::max(
                    result,
                    dirs.iter().sum::<usize>() + dirs.len() + s.len() - depth,
                );
            } else {
                dirs.push(s.len() - depth);
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0388() {
        assert_eq!(
            20,
            Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string())
        );
        assert_eq!(32, Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()));
        assert_eq!(0, Solution::length_longest_path("a".to_string()));
        assert_eq!(
            12,
            Solution::length_longest_path("file1.txt\nfile2.txt\nlongfile.txt".to_string())
        );
    }
}
