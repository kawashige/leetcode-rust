pub struct Solution {}

impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        s.split_whitespace()
            .map(|x| {
                if "aeiouAEIOU".chars().any(|c| x.starts_with(c)) {
                    format!("{}ma", x)
                } else {
                    format!("{}{}ma", &x[1..], &x[0..1])
                }
            })
            .enumerate()
            .map(|(i, x)| format!("{}{}", x, "a".repeat(i + 1)))
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_day19() {
        assert_eq!(
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string(),
            Solution::to_goat_latin("I speak Goat Latin".to_string())
        );

        assert_eq!(
            "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa".to_string(),
            Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string())
        );

        assert_eq!("amaa".to_string(), Solution::to_goat_latin("a".to_string()));
        assert_eq!("Eachmaa ordwmaaa onsistscmaaaa ofmaaaaa owercaselmaaaaaa andmaaaaaaa uppercasemaaaaaaaa etterslmaaaaaaaaa onlymaaaaaaaaaa".to_string(), Solution::to_goat_latin("Each word consists of lowercase and uppercase letters only".to_string()));
    }
}
