pub struct Solution {}

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut count = vec![0; 1001];
        let mut sum = 0;
        for i in 0..skill.len() {
            count[skill[i] as usize] += 1;
            sum += skill[i];
        }
        if sum % (skill.len() as i32 / 2) != 0 {
            return -1;
        }
        sum /= skill.len() as i32 / 2;

        println!("{:?}", sum);

        let mut result = 0;
        for i in 1..=sum / 2 {
            if count[i as usize] == 0 {
                continue;
            }
            if i == sum - i {
                if count[i as usize] % 2 == 1 {
                    return -1;
                }
                result += count[i as usize] as i64 / 2 * i as i64 * i as i64;
            } else {
                if count[i as usize] != count[(sum - i) as usize] {
                    return -1;
                }
                result += count[i as usize] as i64 * i as i64 * (sum - i) as i64;
            }
            println!("{}: {}", i, result);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2491() {
        assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
        assert_eq!(Solution::divide_players(vec![3, 4]), 12);
        assert_eq!(Solution::divide_players(vec![1, 1, 2, 3]), -1);
    }
}
