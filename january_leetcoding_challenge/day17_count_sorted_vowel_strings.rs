pub struct Solution {}

impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut counts = [1; 5];
        for _ in 1..n {
            let mut new_counts = [0; 5];
            let mut sum = 0;
            for i in 0..5 {
                sum += counts[i];
                new_counts[i] = sum;
            }
            counts = new_counts;
        }
        counts.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day17() {
        assert_eq!(5, Solution::count_vowel_strings(1));
        assert_eq!(15, Solution::count_vowel_strings(2));
        assert_eq!(66045, Solution::count_vowel_strings(33));
    }
}
