fn main() {}

const NON_NUM: i32 = 10;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut is_negative = false;
        let mut is_started = false;
        let mut sum: i32 = 0;

        for (i, c) in s.chars().enumerate() {
            if !is_started {
                match c {
                    '-' => {
                        is_negative = true;
                        is_started = true;
                        continue;
                    }
                    '+' => {
                        is_started = true;
                        continue;
                    }
                    ' ' => {
                        continue;
                    }
                    _ => {}
                }

                is_started = true;
            }

            let n = match c.to_digit(10) {
                Some(result) => result as i32,
                None => NON_NUM,
            };

            if n == NON_NUM {
                break;
            }

            match sum.checked_mul(10) {
                Some(result) => sum = result,
                None => {
                    if is_negative {
                        return i32::MIN;
                    } else {
                        return i32::MAX;
                    }
                }
            }

            let result = if is_negative {
                sum.checked_sub(n)
            } else {
                sum.checked_add(n)
            };

            match result {
                Some(r) => sum = r,
                None => {
                    if is_negative {
                        return i32::MIN;
                    } else {
                        return i32::MAX;
                    }
                }
            }

            if n == 0 || i == s.len() - 1 {
                continue;
            }
        }

        sum
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case("42", 42)]
#[case("   -042", -42)]
#[case("1337c0d3", 1337)]
#[case("0-1", 0)]
#[case("words and 987", 0)]
#[case("-91283472332", -2147483648)]
#[case("+1", 1)]
#[case("2147483648", 2147483647)]
fn solution_test(#[case] x: String, #[case] expected: i32) {
    let actual = Solution::my_atoi(x);
    assert_eq!(actual, expected);
}
