pub struct Solution {}

impl Solution {
    pub fn valid_ip_address(ip: String) -> String {
        fn validate_ipv4(ip: &str) -> bool {
            let v = ip.split(".").collect::<Vec<&str>>();
            v.len() == 4
                && v.into_iter().all(|x| {
                    let parsed = x.parse::<i32>().unwrap_or(-1);
                    !(x.len() != 1 && x.starts_with("0")) && 0 <= parsed && parsed < 256
                })
        }

        fn validate_ipv6(ip: &str) -> bool {
            let v = ip.split(":").collect::<Vec<&str>>();
            v.len() == 8
                && v.into_iter().all(|x| {
                    (0 < x.len() && x.len() < 5) && x.chars().all(|c| c.is_ascii_hexdigit())
                })
        }

        if ip.contains(".") {
            if validate_ipv4(&ip) {
                return "IPv4".to_string();
            }
        } else if ip.contains(":") {
            if validate_ipv6(&ip) {
                return "IPv6".to_string();
            }
        }
        "Neither".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_0468() {
        assert_eq!(
            "IPv4".to_string(),
            Solution::valid_ip_address("172.16.254.1".to_string())
        );
        assert_eq!(
            "IPv6".to_string(),
            Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string())
        );
        assert_eq!(
            "Neither".to_string(),
            Solution::valid_ip_address("256.256.256.256".to_string())
        );
        assert_eq!(
            "Neither".to_string(),
            Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334:".to_string())
        );
        assert_eq!(
            "Neither".to_string(),
            Solution::valid_ip_address("1e1.4.5.6".to_string())
        );
        assert_eq!(
            "Neither".to_string(),
            Solution::valid_ip_address(".a.".to_string())
        );
    }
}
