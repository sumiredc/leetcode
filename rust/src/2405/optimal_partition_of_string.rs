/// 
/// @see: https://leetcode.com/problems/optimal-partition-of-string/
/// 

use std::collections::BTreeMap;

impl Solution {
    pub fn partition_string(mut s: String) -> i32 {
        let mut map = BTreeMap::new();
        let mut cnt = 1;

        while s.len() > 0 {
            let c = s.pop().unwrap();
            if map.contains_key(&c) {
                cnt += 1;
                map = BTreeMap::new();
            }
            map.insert(c, false);   
        }

        cnt
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case("abacaba", 4)]
#[case("ssssss", 6)]

fn partition_string_test(#[case] s: String, #[case] expected: i32) {
    let actual = Solution::partition_string(s);
    assert_eq!(actual, expected);
}

fn main() {}
