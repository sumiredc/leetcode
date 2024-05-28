/// 
/// @see: https://leetcode.com/problems/determine-the-minimum-sum-of-a-k-avoiding-array/
/// 
/// @time_complexity  O(N)
/// @space_complexity O(N)
/// 
/// 1. 等差数列の和を算出
/// 2. 減算する等差数列の和を算出
/// 3. 加算する等差数列の和を算出
/// 
impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        // 等差数列の和 を算出
        let a = 1; // 初項
        let sum = Solution::sum_of_arithmetic_sequence(&a, &n, &n);

        // 中央値（繰り上げ）より小さければ、追加計算不要
        if n < (k + 1) >> 1 {
            return sum
        }

        // 中央値(切り捨て)を算出
        let mdn = k >> 1;

        // 除算する 等差数列の和 を算出
        let sub_a = mdn + 1;
        let sub_l = (k - 1).min(n);
        let change_n = sub_l - sub_a + 1;

        let sub_sum = Solution::sum_of_arithmetic_sequence(&sub_a, &sub_l, &change_n);

        // 追加する 等差数列の和 を算出
        let add_a = (n + 1).max(k);
        let add_l = add_a + change_n - 1;
        let add_sum = Solution::sum_of_arithmetic_sequence(&add_a, &add_l, &change_n);

        sum - sub_sum + add_sum
    }

    pub fn sum_of_arithmetic_sequence(a: &i32, l: &i32, n: &i32) -> i32 {
        (*a + *l) * n / 2
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(5, 4, 18)]
#[case(2, 6, 3)]
#[case(3, 3, 8)]
#[case(2, 3, 4)]
#[case(1, 1, 1)]
#[case(2, 1, 3)]
#[case(2, 2, 3)]
#[case(3, 5, 8)]
#[case(4, 5, 14)]

fn sum_of_arithmetic_sequence_test(#[case] n: i32, #[case] k: i32, #[case] expected: i32) {
    let actual = Solution::minimum_sum(n, k);
    assert_eq!(actual, expected);
}

fn main() {}
