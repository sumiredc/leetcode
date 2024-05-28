/// 
/// 等差数列の和
/// 
/// a: 初項
/// d: 公差
/// n: 項数
/// 
impl Solution {
    pub fn sum_of_arithmetic_sequence(a: i32, d: i32, n: i32) -> i32 {
        // 末項
        // 項数 - 1 に 公差を掛け 初項を足す
        let l = (n - 1) * d + a;
        
        // 初項 + 末項 に 項数を掛け 半分にする
        (a + l) * n / 2
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(1, 3, 5, 35)]
#[case(10, 8, 4, 88)]
#[case(4, 1, 10, 85)]

fn sum_of_arithmetic_sequence_test(#[case] a: i32, #[case] d: i32, #[case] n: i32, #[case] expected: i32) {
    let actual = Solution::sum_of_arithmetic_sequence(a, d, n);
    assert_eq!(actual, expected);
}

fn main() {}
