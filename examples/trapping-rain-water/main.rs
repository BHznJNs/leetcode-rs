struct Solution;
impl Solution {
    pub fn trap(heights: Vec<i32>) -> i32 {
        #[inline]
        fn max(a: i32, b: i32) -> i32 {
            if a > b { a } else { b }
        }
        #[inline]
        fn min(a: i32, b: i32) -> i32 {
            if a < b { a } else { b }
        }

        if heights.is_empty() || heights.len() == 1 {
            return 0;
        }

        let max_index = heights.len() - 1;

        let mut start: usize = 0;
        let mut is_initialized = false;
        if heights[0] > heights[1] {
            is_initialized = true;
            start = 0;
        } else {
            let mut temp = 1;
            while temp <= max_index {
                if heights[temp] > heights[temp - 1] {
                    temp += 1;
                } else {
                    is_initialized = true;
                    start = temp - 1;
                    break;
                }
            }
        }
        if !is_initialized {
            return 0;
        }

        let mut trapped_water = 0;
        let mut l = start;

        while l < max_index - 1 {
            let l_height = heights[l];
            let mut max_index = l + 1;
            let mut max_height = 0;
            for (index, height) in heights[(l + 1)..].iter().enumerate() {
                if *height > max_height {
                    max_index  = index + l + 1;
                    max_height = *height;

                    if max_height > l_height {
                        break;
                    }
                }
            }

            let dam = min(heights[l], heights[max_index]);
            let mut trapped_water_temp = 0;
            for m in &heights[(l + 1)..max_index] {
                trapped_water_temp += max(0, dam - *m);
            }
            l = max_index;
            trapped_water += trapped_water_temp;
        }

        return trapped_water;
    }
}

fn main() {
    // println!("{}", Solution::trap(vec![0, 1, 2, 3]));
    // println!("{}", Solution::trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]));

    //           #
    // #         #
    // #     #   #
    // # #   # # #
    // # # _ # # #
    // println!("{}", Solution::trap(vec![4,2,0,3,2,5]));

    // 3 + 4 + 7
    println!("{}", Solution::trap(vec![8,5,4,1,8,9,3,0,0]));
}
