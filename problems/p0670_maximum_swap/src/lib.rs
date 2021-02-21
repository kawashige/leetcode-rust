pub struct Solution {}

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut chars = num.to_string().chars().collect::<Vec<char>>();
        let mut indices = [0; 10];

        for i in (1..chars.len()).rev() {
            if indices[chars[i] as usize - 0x30] == 0 {
                indices[chars[i] as usize - 0x30] = i;
            }
        }

        for i in 0..(chars.len() - 1) {
            if let Some(j) = ((chars[i] as usize - 0x30 + 1)..10)
                .rev()
                .find(|j| i < indices[*j])
            {
                chars.swap(i, indices[j]);
                break;
            }
        }

        chars.into_iter().collect::<String>().parse().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0670() {
        assert_eq!(Solution::maximum_swap(7999), 9997);
        assert_eq!(Solution::maximum_swap(2736), 7236);
        assert_eq!(Solution::maximum_swap(9973), 9973);
    }
}
