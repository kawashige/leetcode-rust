pub struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() == 0 {
            return;
        }

        let mut red = 0;
        let mut blue = nums.len() - 1;
        for i in 0..nums.len() {
            println!(
                "i: {}, nums[i]: {}, red: {}, blue: {}, nums: {:?}",
                i, nums[i], red, blue, nums
            );
            loop {
                match nums[i] {
                    0 => {
                        if red <= i && red != nums.len() - 1 {
                            let tmp = nums[red];
                            nums[red] = nums[i];
                            nums[i] = tmp;
                            while nums[red] == 0 && red < nums.len() - 1 {
                                red += 1;
                            }
                        } else {
                            break;
                        }
                    }
                    2 => {
                        if i <= blue && blue != 0 {
                            let tmp = nums[blue];
                            nums[blue] = nums[i];
                            nums[i] = tmp;
                            while nums[blue] == 2 && blue > 0 {
                                blue -= 1;
                            }
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0075() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut input);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], input);

        let mut input2 = vec![2, 2, 1, 1, 0, 0];
        Solution::sort_colors(&mut input2);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], input2);

        let mut input3 = vec![];
        Solution::sort_colors(&mut input3);
        assert_eq!(Vec::new() as Vec<i32>, input3);

        let mut input4 = vec![2];
        Solution::sort_colors(&mut input4);
        assert_eq!(vec![2], input4);

        let mut input5 = vec![0];
        Solution::sort_colors(&mut input5);
        assert_eq!(vec![0], input5);

        let mut input6 = vec![1];
        Solution::sort_colors(&mut input6);
        assert_eq!(vec![1], input6);

        let mut input7 = vec![2, 1, 0, 1, 0, 2, 1, 2, 1];
        Solution::sort_colors(&mut input7);
        assert_eq!(vec![0, 0, 1, 1, 1, 1, 2, 2, 2], input7);

        let mut input8 = vec![1, 2, 0];
        Solution::sort_colors(&mut input8);
        assert_eq!(vec![0, 1, 2], input8);

        let mut input9 = vec![0, 0];
        Solution::sort_colors(&mut input9);
        assert_eq!(vec![0, 0], input9);
    }
}
