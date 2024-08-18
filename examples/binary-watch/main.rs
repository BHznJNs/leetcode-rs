fn remover(list: &Vec<i32>, remove: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        start: usize,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        list: &Vec<i32>,
        remove: i32,
    ) {
        if path.len() == list.len() - (remove as usize) {
            result.push(path.clone());
        }

        for i in start..list.len() {
            path.push(list[i]);
            backtrack(i + 1, path, result, list, remove);
            path.pop();
        }
    }

    let mut result = vec![];
    backtrack(0, &mut vec![], &mut result, list, remove);
    return result;
}

fn add_leading_zero(val: &str) -> String {
    if val.len() == 1 {
        format!("0{val}")
    } else {
        val.to_owned()
    }
}

fn get_value(bits: Vec<i32>) -> i32 {
    let mut value = 0;
    for i in bits {
        value += 1 << i;
    }
    return value;
}

struct Solution;
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = vec![];

        if turned_on > 8 {
            return result;
        }

        let h_bits = (0..4).collect::<Vec<_>>();
        let m_bits = (0..6).collect::<Vec<_>>();
        for h in 0..=turned_on.min(4) {
            let m = turned_on - h;
            let h_res = remover(&h_bits, 4 - h)
                .into_iter()
                .map(|bits| get_value(bits))
                .filter(|v| *v < 12)
                .map(|v| v.to_string())
                .collect::<Vec<_>>();
            let m_res = remover(&m_bits, 6 - m)
                .into_iter()
                .map(|bits| get_value(bits))
                .filter(|v| *v < 60)
                .map(|v| v.to_string())
                .collect::<Vec<_>>();

            for h in h_res {
                for m in &m_res {
                    result.push(format!("{}:{}", h, add_leading_zero(m)))
                }
            }
        }
        return result;
    }
}

fn main() {
    // let temp1 = vec!["0:03","0:05","0:09","0:17","0:33","0:06","0:10","0:18","0:34","0:12","0:20","0:36","0:24","0:40","0:48","1:01","1:02","1:04","1:08","1:16","1:32","2:01","2:02","2:04","2:08","2:16","2:32","4:01","4:02","4:04","4:08","4:16","4:32","8:01","8:02","8:04","8:08","8:16","8:32","3:00","5:00","9:00","6:00","10:00","12:00"];
    // let temp2 = vec!["0:03","0:05","0:06","0:09","0:10","0:12","0:17","0:18","0:20","0:24","0:33","0:34","0:36","0:40","0:48","1:01","1:02","1:04","1:08","1:16","1:32","2:01","2:02","2:04","2:08","2:16","2:32","3:00","4:01","4:02","4:04","4:08","4:16","4:32","5:00","6:00","8:01","8:02","8:04","8:08","8:16","8:32","9:00","10:00"];
    
    // println!("{:#?}", remover(&vec![0, 1, 2, 3], 2));

    // println!("{:?}", Solution::read_binary_watch(1));
}
