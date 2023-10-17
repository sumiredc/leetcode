/// @see https://leetcode.com/problems/remove-duplicates-from-sorted-array/
///
/// @time_complexity O(N)
/// @space_complexity O(N)
///
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        while 1 < len {
            let current = len - 1;
            if nums[current] == nums[current - 1] {
                nums.remove(current);
            }
            len -= 1;
        }
        nums.len() as i32
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(vec![1,1,2], 2, vec![1,2])]
#[case(vec![0,0,1,1,1,2,2,3,3,4], 5, vec![0,1,2,3,4])]
fn example_test(#[case] mut nums: Vec<i32>, #[case] expected: i32, #[case] nums2: Vec<i32>) {
    let actual = Solution::remove_duplicates(&mut nums);
    assert_eq!(actual, expected);
    assert_eq!(nums, nums2);
}

fn main() {}
