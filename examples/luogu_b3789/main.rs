use std::collections::VecDeque;

enum Compare {
    Larger,
    Less,
}

struct Condition {
    compare: Compare,
    than: isize,
}

struct Tree {
    condition: Condition,
    elements: Vec<Tree>,
}

struct Root {
    elements: Vec<Tree>,
}

fn parse_assignment(line: &str) -> isize {
    let number_str = &line.trim_end_matches(';')[4..];
    let number = number_str.parse().unwrap();
    return number;
}
fn parse_if(line: String, lines: &mut VecDeque<String>) {
    // 
}

fn main() {
    use std::io;
    use std::io::BufRead;

    let mut buffer = String::new();
    let mut stdin = io::stdin().lock();

    stdin.read_line(&mut buffer).unwrap();
    let line_count = buffer.trim_end().parse::<usize>().unwrap();

    let mut lines = VecDeque::new();
    for _ in 0..line_count {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        lines.push_back(buffer.trim().to_string());
    }

    let mut results = vec![0];
    while let Some(line) = lines.pop_front() {
        // if line == "}" {
        //     // end if
        // }
        if line.starts_with('y') {
            // assignment
            results[0] = parse_assignment(&line);
        }

        // if statement
        parse_if(line, &mut lines);
    }
}