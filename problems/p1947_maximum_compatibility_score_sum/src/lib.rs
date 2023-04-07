pub struct Solution {}

impl Solution {
    pub fn to_bits(answers: &[i32]) -> u8 {
        answers
            .iter()
            .enumerate()
            .fold(0, |acc, (i, a)| if a == &0 { acc } else { acc | 1 << i })
    }

    pub fn recurse(
        i: usize,
        score: i32,
        used: &mut Vec<bool>,
        students: &[u8],
        mentors: &[u8],
        questions: i32,
        max_score: &mut i32,
    ) {
        if i == used.len() {
            *max_score = std::cmp::max(*max_score, score);
            return;
        }

        for j in 0..used.len() {
            if !used[j] {
                used[j] = true;
                Self::recurse(
                    i + 1,
                    score + questions - (students[i] ^ mentors[j]).count_ones() as i32,
                    used,
                    students,
                    mentors,
                    questions,
                    max_score,
                );
                used[j] = false;
            }
        }
    }

    pub fn max_compatibility_sum(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        let questions = students[0].len() as i32;
        let students = students
            .iter()
            .map(|student| Self::to_bits(student))
            .collect::<Vec<_>>();
        let mentors = mentors
            .iter()
            .map(|mentor| Self::to_bits(mentor))
            .collect::<Vec<_>>();
        let mut max_score = 0;

        Self::recurse(
            0,
            0,
            &mut vec![false; mentors.len()],
            &students,
            &mentors,
            questions,
            &mut max_score,
        );

        max_score
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1947() {
        assert_eq!(
            Solution::max_compatibility_sum(
                vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 1]],
                vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]
            ),
            8
        );
        assert_eq!(
            Solution::max_compatibility_sum(
                vec![vec![0, 0], vec![0, 0], vec![0, 0]],
                vec![vec![1, 1], vec![1, 1], vec![1, 1]]
            ),
            0
        );
    }
}
