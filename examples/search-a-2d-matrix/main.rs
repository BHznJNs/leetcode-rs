struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if target < matrix[0][0] {
            return false;
        }
        if target > matrix[matrix.len() - 1][matrix[0].len() - 1] {
            return false;
        }
        
        let mut target_row = None;
        for row in &matrix {
            if row[0] > target {
                break;
            }
            target_row = Some(row);
        }

        let Some(row) = target_row else {
            return false;
        };

        if row.len() < 4 {
            for item in row {
                if *item == target {
                    return true;
                }
            }
            return false;
        }

        let mut start = 0;
        let mut end = row.len() - 1;

        while start <= end {
            use std::cmp::Ordering;
            let mid = (start + end) / 2;
            match row[mid].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Less => start = mid + 1,
                Ordering::Greater => end = mid - 1,
            }
        }
        return false;
    }
}

fn main() {
    let matrix = vec![
        vec![1,3,5,7],
        vec![10,11,16,20],
        vec![23,30,34,60],
    ];

    println!("{}", Solution::search_matrix(matrix.clone(), 13));
    println!("{}", Solution::search_matrix(matrix, 10));

    println!("{}", Solution::search_matrix(vec![vec![1], vec![3]], 2));
}