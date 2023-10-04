/// @see https://leetcode.com/problems/longest-common-prefix/
///
/// Write a function to find the longest common prefix string amongst an array of strings.
/// If there is no common prefix, return an empty string "".
///
/// @time_complexity O(N)
/// @space_complexity O(1)
///
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = strs[0].to_string();
        for s in &strs {
            if prefix.len() == 0 {
                break;
            }
            let new_prefix = match_prefix(&prefix, s);
            prefix = new_prefix.to_string();
        }
        prefix
    }
}

fn match_prefix(p: &String, s: &String) -> String {
    let mut prefix = p.clone();
    while !s.starts_with(&prefix) {
        prefix.pop();
    }
    prefix
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(vec!["flower","flow","flight"], "fl")]
#[case(vec!["dog","racecar","car"], "")]
fn longest_common_prefix_test(#[case] strs: Vec<&str>, #[case] expected: &str) {
    let strings = strs.iter().map(|s| s.to_string()).collect();
    let actual = Solution::longest_common_prefix(strings);
    assert_eq!(actual, expected.to_string());
}

fn main() {}
