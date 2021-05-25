pub struct Solution {}

impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let age_count = ages.into_iter().fold(vec![0; 121], |mut age_count, age| {
            age_count[age as usize] += 1;
            age_count
        });

        let mut count = 0;
        for i in 2..121 {
            if age_count[i] == 0 {
                continue;
            }
            let mut sum = 0;
            for j in (i / 2 + 8)..i {
                sum += age_count[j];
            }
            if i > 14 {
                sum += age_count[i] - 1;
            }
            count += sum * age_count[i];
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0825() {
        assert_eq!(
            Solution::num_friend_requests(vec![
                109, 6, 101, 18, 13, 20, 106, 110, 110, 108, 55, 89, 81, 56, 84, 37, 71, 51, 88,
                70, 27, 16, 31, 63, 99, 68, 12, 120, 104, 18, 110, 42, 93, 106, 99, 63, 3, 82, 22,
                17, 69, 49, 88, 117, 57, 37, 95, 15, 50, 18, 77, 103, 102, 104, 87, 1, 23, 97, 93,
                15, 9, 35, 59, 103, 118, 23, 52, 66, 86, 7, 40, 33, 60, 4, 113, 43, 21, 58, 31,
                120, 71, 56, 19, 67, 68, 32, 84, 14, 50, 55, 98, 34, 89, 32, 58, 20, 93, 37, 95,
                40
            ]),
            1872
        );
        assert_eq!(Solution::num_friend_requests(vec![16, 16]), 2);
        assert_eq!(Solution::num_friend_requests(vec![16, 17, 18]), 2);
        assert_eq!(
            Solution::num_friend_requests(vec![20, 30, 100, 110, 120]),
            3
        );
    }
}
