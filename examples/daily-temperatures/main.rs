struct Solution;


// struct Stack {
//     list: Vec<i32>,
// }
// impl Stack {
//     pub fn new() -> Self {
//         Self {
//             list: vec![]
//         }
//     }
//     pub fn push(&mut self, val: i32) {
//         self.list.push(val)
//     }
//     pub fn pop(&mut self) -> Option<i32> {
//         self.list.pop()
//     }
//     pub fn top(&self) -> Option<i32> {
//         self.list.last().copied()
//     }
// }

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        if temperatures.len() == 1 {
            return vec![0];
        }

        let mut result_list = vec![0; temperatures.len()];
        for i in 1..temperatures.len() {
            let cur_temp = temperatures[i];
            let mut step = 0;
            for j in (0..i).rev() {
                let before_temp = temperatures[j];
                step += 1;

                if cur_temp <= before_temp {
                    break;
                }
                if result_list[j] > 0 {
                    continue;
                }
                result_list[j] = step;
            }
        }

        return result_list;
    }
}

fn main() {
    // [73,74,75,71,69,72,76,73]
    // [1 ,1, 4, 2, 1, 1, 0, 0 ]
    // println!("{:?}", Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]));
    println!("{:?}", Solution::daily_temperatures(vec![89,62,70,58,47,47,46,76,100,70]));
}
