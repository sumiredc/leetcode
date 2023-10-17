/// @see https://leetcode.com/problems/remove-duplicates-from-sorted-array/
///
/// @time_complexity O(N)
/// @space_complexity O(1)
///
impl Solution {
    // Runtime 0ms, Memory 2.3MB
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }

    // Runtime 2ms, Memory 2.1MB
    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut i = 0;
        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                i += 1;
                nums[i] = nums[j];
            }
        }
        nums.drain(i..nums.len() - 1);
        (i + 1) as i32
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(vec![1,1,2], 2, vec![1,2])]
#[case(vec![0,0,1,1,1,2,2,3,3,4], 5, vec![0,1,2,3,4])]
fn remove_duplicates_test(
    #[case] mut nums: Vec<i32>,
    #[case] expected: i32,
    #[case] nums2: Vec<i32>,
) {
    let actual = Solution::remove_duplicates(&mut nums);
    assert_eq!(actual, expected);
    assert_eq!(nums, nums2);

    let actual = Solution::remove_duplicates2(&mut nums);
    assert_eq!(actual, expected);
    assert_eq!(nums, nums2);
}

fn main() {}
