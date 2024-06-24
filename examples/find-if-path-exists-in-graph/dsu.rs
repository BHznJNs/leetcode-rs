struct Solution;
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {        
        fn find(pre: &Vec<i32>, target: i32) -> i32 {
            let parent = pre[target as usize];
            if parent == target {
                return target;
            }
            return find(pre, parent);
        }
        fn union(pre: &mut Vec<i32>, depth: &mut Vec<i32>, u: i32, v: i32) {
            use std::cmp::Ordering;

            let u_parent = find(pre, u);
            let v_parent = find(pre, v);
            let u_depth = depth[u as usize];
            let v_depth = depth[v as usize];

            match u_depth.cmp(&v_depth) {
                Ordering::Less    => pre[u_parent as usize] = v_parent,
                Ordering::Greater => pre[v_parent as usize] = u_parent,
                Ordering::Equal => {
                    pre[v_parent as usize] = u_parent;
                    depth[u_parent as usize] += 1;
                }
            }
        }
        
        let mut pre = (0..=n).collect::<Vec<i32>>();
        let mut depth = vec![1; n as usize];

        for edge in edges {
            let [u, v] = edge[..] else { unreachable!() };
            union(&mut pre, &mut depth, u, v);
        }

        return find(&pre, source) == find(&pre, destination);
    }
}

#[test]
fn test() {
    println!(
        "{}",
        Solution::valid_path(
            10,
            vec![
                [4, 3].to_vec(),
                [1, 4].to_vec(),
                [4, 8].to_vec(),
                [1, 7].to_vec(),
                [6, 4].to_vec(),
                [4, 2].to_vec(),
                [7, 4].to_vec(),
                [4, 0].to_vec(),
                [0, 9].to_vec(),
                [5, 4].to_vec()
            ],
            5,
            9,
        )
    );
}
