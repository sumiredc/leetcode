/// @see https://leetcode.com/problems/valid-parentheses/
///
///
/// @time_complexity O(N)
/// @space_complexity O(N)
///
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<String> = vec![];
        let symbols = make_symbols();

        for c in s.chars() {
            let str_unit = c.to_string();

            if symbols.contains_key(str_unit.as_str()) {
                // 閉じカッコ

                // stack が 空なら false
                if stack.is_empty() {
                    return false;
                }

                let pair = symbols.get(str_unit.as_str()).unwrap();
                if stack.pop().unwrap() != *pair {
                    return false;
                }
            } else {
                // 開始カッコ
                stack.push(str_unit);
            }
        }

        // stack が 空になっているか検証
        stack.is_empty()
    }
}

fn make_symbols() -> HashMap<String, String> {
    let mut symbols = HashMap::new();
    let brackets = vec![
        ("}".to_string(), "{".to_string()),
        ("]".to_string(), "[".to_string()),
        (")".to_string(), "(".to_string()),
    ];
    symbols.extend(brackets);
    symbols
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case("()", true)]
#[case("()[]{}", true)]
#[case("(]", false)]
fn is_valid_test(#[case] s: &str, #[case] expected: bool) {
    let actual = Solution::is_valid(s.to_string());
    assert_eq!(actual, expected);
}

fn main() {}
