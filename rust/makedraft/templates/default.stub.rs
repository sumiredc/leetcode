/// @see
///
/// @time_complexity O()
/// @space_complexity O()
///
impl Solution {}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case("example", true)]
fn example_test(#[case] s: &str, #[case] expected: bool) {
    let actual = true;
    assert_eq!(actual, expected);
}

fn main() {}
