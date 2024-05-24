struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        #[inline]
        fn insert(result: &mut Vec<Vec<i32>>, value: Vec<i32>) {
            for item in result.iter() {
                if item[0] == value[0] && item[1] == value[1] {
                    return;
                }
            }
            result.push(value);
        }

        let mut origin: Vec<i32> = nums;
        origin.sort();

        let mut result = vec![];

        for i in 0..(origin.len() - 2) {
            let mut j = i + 1;
            let mut k = origin.len() - 1;

            while j < k {
                let i_val = origin[i];
                let j_val = origin[j];
                let k_val = origin[k];
                let sum = i_val + j_val + k_val;

                if sum == 0 {
                    j += 1;
                    insert(&mut result, vec![i_val, j_val, k_val]);
                } else
                if sum < 0 {
                    j += 1;
                } else
                if sum > 0 {
                    k -= 1;
                }
            }
        }
        return result;
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1,0,1,2,-1,-4]))
}