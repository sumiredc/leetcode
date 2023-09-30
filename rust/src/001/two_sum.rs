/// @see https://leetcode.com/problems/two-sum/
///
/// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
/// You may assume that each input would have exactly one solution, and you may not use the same element twice.
/// You can return the answer in any order.
///
/// 2 <= nums.length <= 104
/// -109 <= nums[i] <= 109
/// -109 <= target <= 109
/// Only one valid answer exists.
///
/// @time_complexity     O(N)
/// @space_complexity    O(N)
///
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map = HashMap::new();

        let mut same_vec: Vec<i32> = vec![];

        for (i, n) in nums.iter().enumerate() {
            let index = i as i32;

            // n*2がtarget と一致する
            // -> same_vecへindexを追加
            if *n * 2 == target {
                same_vec.push(index);
                // same_vecが2つ揃っていればクリア
                if same_vec.len() == 2 {
                    return same_vec;
                }
                continue;
            }

            // hash mapにnが存在する
            // -> スキップ
            if hash_map.contains_key(n) {
                continue;
            }

            let diff = target - *n;
            // hash mapに target - n の値が存在するか確認
            // -> 存在すればペア成立として返却
            if hash_map.contains_key(&diff) {
                return vec![*hash_map.get(&diff).unwrap(), index];
            }

            hash_map.insert(*n, index);
        }

        // 該当するペアが存在しない
        vec![]
    }
}

pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(vec![2, 7, 11, 15], 9 , vec![0, 1])]
#[case(vec![3, 2, 4], 6 , vec![1, 2])]
#[case(vec![3, 3], 6 , vec![0, 1])]
#[case(vec![1,1,1,1,1,4,1,1,1,1,1,7,1,1,1,1,1], 11, vec![5, 11])]
fn two_sum_test(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
    let actual = Solution::two_sum(nums, target);
    assert_eq!(actual, expected);
}

fn main() {}
