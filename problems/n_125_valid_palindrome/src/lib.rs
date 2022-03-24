use std::ops::Mul;

struct Solution {}

impl Solution {
    pub fn is_palindrome(str: String) -> bool {
        // Check constraints.
        if str.len() < 1 || str.len() > 2_i32.mul(10_i32.pow(5)) as usize {
            return false
        }

        // Clean string.
        let clean_str: String = str
            .chars()
            .into_iter()
            .map(|c| {
                if c.is_alphanumeric() {
                    return c.to_lowercase().to_string();
                }
                "".to_string()
            })
            .collect();

        // Check is this string is palindrome.
        Solution::check_palindrome(&clean_str)
    }

    fn check_palindrome(str: &str) -> bool {
        match str.len() {
            1 | 0 => return true, // case when we have only one letter in a string (like middle letter) or we have finished dividing and check
            _ => (),
        }
        let left = &str[..1];
        let right = &str[str.len() - 1..];
        // println!("{} | {} | {} | {}", str, left, right, &str[1..str.len() - 1]);
        if left == right {
            return Solution::check_palindrome(&str[1..str.len() - 1]);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let cases: Vec<(&str, bool)> = vec![
            // Positive.
            ("605 + 506", true),
            ("605 5 506", true),
            ("Do geese see God?", true),
            ("Rats live on no evil star.", true),
            ("race car", true),
            ("A man, a plan, a canal: Panama", true),
            (" ", true),
            // Negative.
            ("Hello World!", false),
            ("race a car", false)
        ];
        for case in cases {
            if Solution::is_palindrome(case.0.to_string()) != case.1 {
                return Err(format!("{} is not {}", case.0, case.1).into())
            }
        }
        Ok(())
    }
}
