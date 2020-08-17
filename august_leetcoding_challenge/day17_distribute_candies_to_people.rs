pub struct Solution {}

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut sum = 1;
        let mut n = 1;
        while sum + n + 1 <= candies {
            n += 1;
            sum += n;
        }

        println!("{},{},{}", n, sum, candies);

        let mut result = vec![0; num_people as usize];
        (1..=n).for_each(|v| {
            result[((v - 1) % num_people) as usize] += v;
        });
        result[(n % num_people) as usize] += candies - sum;
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day17() {
        assert_eq!(vec![1, 2, 3, 1], Solution::distribute_candies(7, 4));
        assert_eq!(vec![5, 2, 3], Solution::distribute_candies(10, 3));
        assert_eq!(vec![10], Solution::distribute_candies(10, 1));
    }
}
