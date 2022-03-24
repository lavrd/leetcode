struct Solution {}

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        // If x < 0 it can't be a palindrome because of "-" at the beginning.
        // If last digit of x is 0 then palindrome should start from 0. So it can be only zero.
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut reverted = 0;
        while x > reverted {
            // Get las digit from origin by mod 10; 123%10=3; 874%10=4.
            let last_digit = x % 10;
            // Move last digit to its correct position.
            // 123
            //   0=0*10+3
            //     3=3*10+2
            //       32=32*10+1
            //         321
            reverted = reverted * 10 + last_digit;
            // Remove last digit from origin.
            x /= 10;
        }

        // We need second branch because our reverted number can be odd,
        // so we need to remove one digit from it.
        // 12321 -> 12 == 123 -> 12 == 12.
        x == reverted || x == reverted / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        let cases: Vec<(i32, bool)> = vec![
            // Positive.
            (121, true),
            (2442, true),
            (999, true),
            (12321, true),
            (100001, true),
            // Negative.
            (-121, false),
            (123, false),
            (10, false),
            (128721, false),
            (127821, false),
        ];
        for case in cases {
            if Solution::is_palindrome(case.0) != case.1 {
                return Err(format!("{} is not {}", case.0, case.1).into());
            }
        }
        Ok(())
    }
}
