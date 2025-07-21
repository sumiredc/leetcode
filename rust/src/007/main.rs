fn main() {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut n = x.abs();
        let mut is_negative = false;

        // 負の数の場合はフラグを立て、n を反転させる
        if x < 0 {
            is_negative = true;
        }

        let mut sum: i32 = 0;

        // 1桁ずつ抽出し結果へ加算 & 桁上げを繰り返す
        while n != 0 {
            // 一桁目を抽出
            let remain = n % 10;

            // オーバーフローを考慮した加算
            match sum.checked_add(remain) {
                Some(result) => {
                    sum = result;
                }
                None => {
                    // オーバーフローした時点で 0 確定
                    sum = 0;
                    break;
                }
            };

            // 元の値から一の位を減算
            n -= remain;

            // n が 0 になった時点で後続処理は不要のためループ脱出
            if n == 0 {
                break;
            }

            // 1の位の処理が終わったので、次の位の準備
            n /= 10;

            // 合計が 0 なら後続処理は不要のため次のループへ移動
            if sum == 0 {
                continue;
            }

            // オーバーフローを考慮した乗算（反転結果の桁上げ）
            match sum.checked_mul(10) {
                Some(result) => {
                    sum = result;
                }
                None => {
                    // オーバーフローした時点で 0 確定
                    sum = 0;
                    break;
                }
            }
        }

        // 結果 が 0 なら早期リターン
        if sum == 0 {
            return 0;
        }

        // 負の数の場合はオーバーフローを考慮して反転処理を実施
        if is_negative {
            match sum.checked_mul(-1) {
                Some(result) => {
                    sum = result;
                }
                None => {
                    sum = 0;
                }
            }
        }

        sum
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(123, 321)]
#[case(-123, -321)]
#[case(120, 21)]
#[case(1534236469, 0)]
fn solution_test(#[case] x: i32, #[case] expected: i32) {
    let actual = Solution::reverse(x);
    assert_eq!(actual, expected);
}
