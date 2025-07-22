fn main() {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut vol: i32 = 0;
        let mut i1 = 0;
        let mut i2 = height.len() - 1;

        while i1 < i2 {
            let w = (i2 - i1) as i32;
            let h1 = height[i1];
            let h2 = height[i2];
            let h = h1.min(h2);

            vol = vol.max(h * w);

            if h1 < h2 {
                i1 += 1;
                continue;
            } else if h1 > h2 {
                i2 -= 1;
                continue;
            }

            if height[i1 + 1] < height[i2 - 1] {
                i2 -= 1;
            } else {
                i1 += 1;
            }
        }

        vol
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(vec![1,8,6,2,5,4,8,3,7], 49)]
#[case(vec![1,1], 1)]
fn solution_test(#[case] height: Vec<i32>, #[case] expected: i32) {
    let actual = Solution::max_area(height);
    assert_eq!(actual, expected);
}
