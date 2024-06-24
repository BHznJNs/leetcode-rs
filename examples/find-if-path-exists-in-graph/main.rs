mod dsu;

struct Solution;
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        use std::collections::BTreeMap;

        if source == destination {
            return true;
        }
        if edges.is_empty() {
            return false;
        }

        let size = n as usize;
        let mut graph: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
        // let mut matrix: Vec<Vec<bool>> = vec![vec![false; size]; size];

        for edge in edges {
            let [u, v] = edge[..] else { unreachable!() };
            if let Some(vec) = graph.get_mut(&u) {
                vec.push(v);
            } else {
                graph.insert(u, vec![v]);
            }
            if let Some(vec) = graph.get_mut(&v) {
                vec.push(u);
            } else {
                graph.insert(v, vec![u]);
            }
        }

        let mut visited = vec![false; size];
        fn walk(
            current: i32,
            target: i32,
            graph: &BTreeMap<i32, Vec<i32>>,
            visited: &mut Vec<bool>,
        ) -> bool {
            visited[current as usize] = true;

            for next in graph.get(&current).unwrap().iter() {
                if visited[*next as usize] {
                    continue;
                }
                if target == *next {
                    return true;
                }
                if walk(*next, target, graph, visited) {
                    return true;
                }
            }
            return false;
        }
        return walk(source, destination, &graph, &mut visited);
    }
}

fn main() {
    // println!(
    //     "{}",
    //     Solution::valid_path(
    //         6,
    //         vec![
    //             [0, 1].to_vec(),
    //             [0, 2].to_vec(),
    //             [3, 5].to_vec(),
    //             [5, 4].to_vec(),
    //             [4, 3].to_vec()
    //         ],
    //         0,
    //         5
    //     )
    // );
    // println!("{}", Solution::valid_path(5, vec![[0, 4].to_vec(),], 0, 4,));
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
    // println!(
    //     "{}",
    //     Solution::valid_path(
    //         1,
    //         vec![],
    //         0,
    //         0,
    //     )
    // );
}
