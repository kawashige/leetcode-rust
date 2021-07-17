pub struct Solution {}

impl Solution {
    pub fn to_str(arr: &[i32]) -> String {
        arr.iter()
            .skip_while(|x| x == &&0)
            .map(|i| (b'0' + *i as u8) as char)
            .collect::<String>()
    }

    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; arr.len()];
        *count.last_mut().unwrap() = *arr.last().unwrap();
        for i in (0..(arr.len() - 1)).rev() {
            count[i] = count[i + 1] + arr[i];
        }

        let total = count[0];
        if total == 0 {
            return vec![0, arr.len() as i32 - 1];
        } else if total % 3 != 0 {
            return vec![-1, -1];
        }
        let part_count = total / 3;

        let trailing_zeros = count.iter().rev().take_while(|x| x == &&0).count();

        let second_e = (0..count.len())
            .rev()
            .find(|i| count[*i] == part_count + 1)
            .unwrap()
            + trailing_zeros;
        if second_e + 1 >= count.len() || count[second_e + 1] != part_count {
            return vec![-1, -1];
        }

        let first_e = (0..second_e)
            .rev()
            .find(|i| count[*i] == part_count * 2 + 1)
            .unwrap()
            + trailing_zeros;
        if first_e + 1 >= count.len() || count[first_e + 1] != part_count * 2 {
            return vec![-1, -1];
        }

        let strings = vec![
            Self::to_str(&arr[..=first_e]),
            Self::to_str(&arr[(first_e + 1)..=second_e]),
            Self::to_str(&arr[(second_e + 1)..]),
        ];

        if strings[0] == strings[1] && strings[0] == strings[2] {
            vec![first_e as i32, second_e as i32 + 1]
        } else {
            vec![-1, -1]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day17() {
        assert_eq!(
            Solution::three_equal_parts(vec![
                1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0,
                0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0
            ]),
            vec![15, 32]
        );
        assert_eq!(Solution::three_equal_parts(vec![1, 0, 1, 0, 1]), vec![0, 3]);
        assert_eq!(
            Solution::three_equal_parts(vec![1, 1, 0, 1, 1]),
            vec![-1, -1]
        );
        assert_eq!(Solution::three_equal_parts(vec![1, 1, 0, 0, 1]), vec![0, 2]);
    }
}
