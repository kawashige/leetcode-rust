pub struct Solution {}

impl Solution {
    pub fn find_kth_smallest_number(
        count: &mut [usize; 10],
        remains: &mut i32,
        digits: &mut Vec<usize>,
    ) -> bool {
        if count.iter().all(|d| d == &0) {
            *remains -= 1;
            return remains == &0;
        }

        for d in 0..count.len() {
            if count[d] == 0 {
                continue;
            }
            digits.push(d);
            count[d] -= 1;
            if Self::find_kth_smallest_number(count, remains, digits) {
                return true;
            }
            count[d] += 1;
            digits.pop();
        }

        false
    }

    pub fn count_swaps(org: &[usize], new: &[usize]) -> i32 {
        print!("org: {:?}, new: {:?}", org, new);
        let mut new = new.to_vec();
        let mut count = 0;
        for i in 0..new.len() {
            if org[i] != new[i] {
                for j in i + 1..new.len() {
                    if org[i] == new[j] {
                        for k in (i..j).rev() {
                            new.swap(k, k + 1);
                            count += 1;
                        }
                        break;
                    }
                }
            }
        }

        count
    }

    pub fn get_min_swaps(num: String, k: i32) -> i32 {
        let digits = num
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        let mut count = [0; 10];
        count[*digits.last().unwrap()] += 1;

        let mut remains = k;
        for i in (0..digits.len() - 1).rev() {
            println!("i: {}, digits[i]: {}: count: {:?}", i, digits[i], count);
            count[digits[i]] += 1;
            for j in digits[i] + 1..count.len() {
                if count[j] == 0 {
                    continue;
                }
                let mut new_digits = vec![j];
                count[j] -= 1;
                if Self::find_kth_smallest_number(&mut count, &mut remains, &mut new_digits) {
                    println!("{:?}", new_digits);
                    return Self::count_swaps(
                        &digits[digits.len() - new_digits.len()..],
                        &new_digits,
                    );
                }
                count[j] += 1;
            }
        }

        unreachable!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1850() {
        assert_eq!(Solution::get_min_swaps("5489355142".to_string(), 4), 2);
        assert_eq!(Solution::get_min_swaps("11112".to_string(), 4), 4);
        assert_eq!(Solution::get_min_swaps("00123".to_string(), 1), 1);
    }
}
