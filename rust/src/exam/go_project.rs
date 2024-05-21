/// あなたは今、非常に単調なRPGゲームで魔王を倒そうとしています。魔王のHPは h です。このゲームはターン制で、1 ターンは以下のように進みます。
/// あなたが魔王に攻撃する。魔王のHPが a 減る
/// もし魔王のHPが 0 以下になった場合、魔王を倒したことになる。
/// 魔王は回復魔法を使うので、魔王のHPが b 増える
/// 魔王を倒せるでしょうか？また、もし倒せるならば何ターンかかるかを計算するプログラムを書いてください。
///
/// ※ 標準入力は下記のように渡される
/// 10 5 2
///
use std::error::Error;
use std::io::{self, BufRead};

const LOSE: &str = "NO";
const WIN: &str = "OK";

fn main() -> Result<(), Box<dyn Error>> {
    // 標準入力を取得
    let stdin = io::stdin();

    // 1行読み取り、空白で区切られた値をベクターに格納
    let mut input_line = String::new();
    stdin
        .lock()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let values: Vec<&str> = input_line.split_whitespace().collect();

    // 各変数に分配
    let h: usize = values[0].parse().unwrap();
    let a: usize = values[1].parse().unwrap();
    let b: usize = values[2].parse().unwrap();

    // 判定処理（テストのために関数化）
    let (result, turn) = buttle_mao(h, a, b);

    // 標準出力
    println!("{}", &result);
    if result == WIN {
        println!("{}", &turn);
    }

    Ok(())
}

///
/// @time_complexity     O(1)
/// @space_complexity    O(1)
///
fn buttle_mao(h: usize, a: usize, b: usize) -> (String, usize) {
    // damage > heal  -> WIN
    // damage > HP    -> WIN
    let r = a > b || a >= h;

    // lose
    if !r {
        return (LOSE.to_string(), 0);
    }

    // damage >= h -> 1 kill
    if a >= h {
        return (WIN.to_string(), 1);
    }

    // damage - heal
    let turn_damage = a - b;

    // Boss remain HP at last turn
    let mut last_damage = h % turn_damage;
    if last_damage == 0 {
        last_damage = turn_damage;
    }

    // trun count
    let turn = (h - last_damage) / turn_damage + 1;

    (WIN.to_string(), turn)
}

use rstest::rstest;

#[rstest]
#[case(10, 10, 10, "OK", 1)]
#[case(5, 1, 1, "NO", 0)]
#[case(10, 11, 12, "OK", 1)]
#[case(10, 11, 10, "OK", 1)]
#[case(10, 5, 6, "NO", 0)]
#[case(1, 1, 1, "OK", 1)]
#[case(10, 4, 1, "OK", 4)]
#[case(10, 8, 1, "OK", 2)]
fn is_palindrome_test(
    #[case] h: usize,
    #[case] a: usize,
    #[case] b: usize,
    #[case] expected: String,
    #[case] t: usize,
) {
    let (actual, turn) = buttle_mao(h, a, b);
    assert_eq!(actual, expected);
    assert_eq!(turn, t);
}
