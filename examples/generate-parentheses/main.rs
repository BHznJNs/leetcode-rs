struct Solution;

impl Solution {
    fn generate_parenthesis(n: i32) -> Vec<String> {
        use std::collections::HashSet;
        static mut CACHE: [Option<Vec<String>>; 9] = [
            None,
            None,// Some(vec![String::from("()")]),
            None,// Some(vec![String::from("()()"), String::from("(())")]),
            None,
            None,
            None,
            None,
            None,
            None,
        ];

        unsafe { CACHE[0] = Some(vec![String::from("")]) }
        unsafe { CACHE[1] = Some(vec![String::from("()")]) };
        unsafe { CACHE[2] = Some(vec![String::from("()()"), String::from("(())")]) };

        unsafe {
            if let Some(cached) = &CACHE[n as usize] {
                return cached.clone();
            }
        }

        let mut result_set = HashSet::new();

        let sub = Self::generate_parenthesis(n - 1);
        // (n-1...)
        let middle_condition = sub.iter().map(|str| format!("({})", str));
        result_set.extend(middle_condition);

        for i in 1..n {
            let sub1 = Self::generate_parenthesis(i - 1);
            let sub2 = Self::generate_parenthesis(n - i);
            let mut conditions = vec![];

            for i in &sub1 {
                for j in &sub2 {
                    let condition1 = format!("({}){}", i, j);
                    let condition2 = format!("{}({})", j, i);
                    conditions.push(condition1);
                    conditions.push(condition2);
                }
            }
            result_set.extend(conditions);
        }

        let result_vec = result_set.into_iter().collect::<Vec<String>>();
        unsafe { CACHE[n as usize] = Some(result_vec.clone()) };
        return result_vec;
    }
}

fn main() {
    // let temp = ["()((()))","()(()())","()(())()","(((())))","()()(())","()()()()","((()()))","(())()()","(()(()))","(()()())","((())())","((()))()","(()())()"];
    // let temp = ["(((())))","((()()))","((())())","((()))()","(()(()))","(()()())","(()())()","(())(())","(())()()","()((()))","()(()())","()(())()","()()(())","()()()()"];
    println!("{:?}", Solution::generate_parenthesis(3));
}
