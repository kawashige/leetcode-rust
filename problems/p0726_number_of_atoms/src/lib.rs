use std::collections::BTreeMap;

pub struct Solution {}

impl Solution {
    pub fn parse_atom(i: usize, formula: &[char]) -> (String, usize) {
        let mut j = i + 1;
        while j < formula.len() && formula[j].is_ascii_lowercase() {
            j += 1;
        }
        (formula[i..j].iter().collect::<String>(), j)
    }
    pub fn parse_count(i: usize, formula: &[char]) -> (usize, usize) {
        if i == formula.len() || !formula[i].is_numeric() {
            return (1, i);
        }
        let mut j = i + 1;
        while j < formula.len() && formula[j].is_numeric() {
            j += 1;
        }
        (
            formula[i..j]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
            j,
        )
    }

    pub fn count_of_atoms(formula: String) -> String {
        let formula = formula.chars().collect::<Vec<_>>();
        let mut stack = vec![vec![]];

        let mut i = 0;
        while i < formula.len() {
            if formula[i].is_ascii_uppercase() {
                let (atom, j) = Self::parse_atom(i, &formula);
                let (count, j) = Self::parse_count(j, &formula);
                stack.last_mut().unwrap().push((atom, count));
                i = j;
            } else if formula[i] == '(' {
                stack.push(Vec::new());
                i = i + 1;
            } else if formula[i] == ')' {
                let (count, j) = Self::parse_count(i + 1, &formula);
                for (a, c) in stack.pop().unwrap() {
                    stack.last_mut().unwrap().push((a, c * count));
                }
                i = j;
            }
        }

        let mut map = BTreeMap::new();
        for (a, c) in stack.pop().unwrap() {
            *map.entry(a).or_insert(0) += c;
        }
        map.into_iter()
            .map(|(a, c)| if c == 1 { a } else { format!("{}{}", a, c) })
            .collect::<String>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0726() {
        assert_eq!(
            Solution::count_of_atoms("H2O".to_string()),
            "H2O".to_string()
        );
        assert_eq!(
            Solution::count_of_atoms("Mg(OH)2".to_string()),
            "H2MgO2".to_string()
        );
        assert_eq!(
            Solution::count_of_atoms("K4(ON(SO3)2)2".to_string()),
            "K4N2O14S4".to_string()
        );
    }
}
