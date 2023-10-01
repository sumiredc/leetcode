/// @see https://leetcode.com/problems/palindrome-number/
///
/// @time_complexity     O(N)
/// @space_complexity    O(1)
///
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        // 文字列の配列に変換
        let chars: Vec<char> = x.to_string().chars().collect();

        // 手前・逆から順に一致するか検証
        for (i, c) in chars.iter().enumerate() {
            if c != &chars[chars.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(121, true)]
#[case(-121, false)]
#[case(10, false)]
fn is_palindrome_test(#[case] x: i32, #[case] expected: bool) {
    let actual = Solution::is_palindrome(x);
    assert_eq!(actual, expected);
}

fn main() {}
