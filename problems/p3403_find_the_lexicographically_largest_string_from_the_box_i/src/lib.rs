pub struct Solution {}

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }

        let l = word.len() - (num_friends as usize - 1);
        let mut target = &word[0..l];

        for i in 0..word.len() {
            if target < &word[i..(i + l).min(word.len())] {
                target = &word[i..(i + l).min(word.len())];
            }
        }

        target.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_3403() {
        assert_eq!(
            Solution::answer_string("dbca".to_string(), 2),
            "dbc".to_string()
        );
        assert_eq!(
            Solution::answer_string("gggg".to_string(), 4),
            "g".to_string()
        );
    }
}
