struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        let mut nums = nums;
        nums.sort();

        let max_index = nums.len() - 1;
        let mut triplets = vec![];

        for l in 0..=(max_index-2) {
            let l_num = nums[l];
            if l > 0 && l_num == nums[l - 1] {
                continue;
            }

            let mut m = l + 1;
            let mut r = max_index;

            while m < r {
                let m_num = nums[m];
                let r_num = nums[r];

                let sum = l_num + m_num + r_num;
                match sum.cmp(&0) {
                    Ordering::Less => m += 1,
                    Ordering::Greater => r -= 1,
                    Ordering::Equal => {
                        triplets.push(vec![l_num, m_num, r_num]);
                        for num in &nums[m+1..r] {
                            if *num == m_num {
                                m += 1;
                            } else {
                                break;
                            }
                        }
                        m += 1;
                    }
                }
            }
        }
        return triplets;
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![0,0,0]));
    // println!("{:?}", Solution::three_sum(vec![-1,0,1,2,-1,-4]));
    // println!("{:?}", Solution::three_sum(vec![-2,0,0,2,2]));
    // println!("{:?}", Solution::three_sum(vec![-1,0,1,2,-1,-4,-2,-3,3,0,4]));
}