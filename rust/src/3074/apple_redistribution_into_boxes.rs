///
/// @see https://leetcode.com/problems/apple-redistribution-into-boxes/
/// 
/// N = capacity length
/// @time_complexity O(NlogN)
/// @space_complexity O(NlogN)
/// 
impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut apple_count = apple.iter().sum::<i32>();
        capacity.sort_by(|a, b| b.cmp(a));

        let mut need_box_count = 0;
        for n in capacity {
            need_box_count += 1;
            apple_count -= n;
            if apple_count <= 0 {
                break;
            }
        }
        
        need_box_count
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(vec![1,3,2], vec![4,3,1,5,2], 2)]
#[case(vec![5,5,5], vec![2,4,2,7], 4)]
fn minimum_boxes_test(
    #[case] apple: Vec<i32>,
    #[case] capacity: Vec<i32>,
    #[case] expected: i32,
) {
    let actual = Solution::minimum_boxes(apple, capacity);
    assert_eq!(actual, expected);
}

fn main() {}
