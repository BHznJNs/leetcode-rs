struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;

        let mut count_map: BTreeMap<i32, usize> = BTreeMap::new();

        for n in &nums {
            count_map
                .entry(*n)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let target_count = nums.len() / 2;
        for (k, v) in count_map {
            if v > target_count {
                return k;
            }
        }
        return 0;
    }
}

fn main() {
    print!("{}", Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}
