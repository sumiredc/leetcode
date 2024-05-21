/// @see https://leetcode.com/problems/roman-to-integer/description/
///
/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
///
/// - I can be placed before V (5) and X (10) to make 4 and 9.
/// - X can be placed before L (50) and C (100) to make 40 and 90.
/// - C can be placed before D (500) and M (1000) to make 400 and 900.
///
/// @time_complexity O(N)
/// @space_complexity O(1)
///
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let symbols = make_symbols();
        let pair_symbols = make_pair_symbols();
        let mut total = 0;
        let mut before = String::new();

        for c in s.chars() {
            let k = c.to_string();
            let bk = format!("{}{}", before, k);
            // 加算
            total += symbols.get(k.as_str()).unwrap();

            if pair_symbols.contains(bk.as_str()) {
                // 減算
                total -= symbols.get(before.as_str()).unwrap() * 2;
            }

            before.clone_from(&k);
        }

        total
    }
}

fn make_symbols() -> HashMap<&'static str, i32> {
    let mut symbols: HashMap<&str, i32> = HashMap::new();

    symbols.extend(
        vec![
            ("I", 1),
            ("V", 5),
            ("X", 10),
            ("L", 50),
            ("C", 100),
            ("D", 500),
            ("M", 1000),
        ],
    );
    symbols
}

fn make_pair_symbols() -> HashSet<&'static str> {
    let mut symbols: HashSet<&str> = HashSet::new();
    symbols.extend(vec!["IV", "IX", "XL", "XC", "CD", "CM"]);
    symbols
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case("III", 3)]
#[case("LVIII", 58)]
#[case("MCMXCIV", 1994)]
fn roman_to_int_test(#[case] s: String, #[case] expected: i32) {
    let actual = Solution::roman_to_int(s);
    assert_eq!(actual, expected);
}

fn main() {}
