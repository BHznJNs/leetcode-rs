struct Solution;

struct Stack<T> {
    list: Vec<T>,
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
    pub fn push(&mut self, val: T) {
        self.list.push(val)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.list.pop()
    }
    pub fn top(&self) -> Option<&T> {
        self.list.last()
    }
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        enum Operator {
            Plus,
            Minus,
            Multiply,
            Divide,
        }

        let mut operand_stack = Stack::<i32>::new();

        for token in tokens {
            let Some(operator) = (match token.as_str() {
                "+" => Some(Operator::Plus),// operator_stack.push(Operator::Plus),
                "-" => Some(Operator::Minus),// operator_stack.push(Operator::Minus),
                "*" => Some(Operator::Multiply),// operator_stack.push(Operator::Multiply),
                "/" => Some(Operator::Divide),// operator_stack.push(Operator::Divide),
                int => {
                    operand_stack.push(int.parse::<i32>().unwrap());
                    None
                }
            }) else {
                continue;
            };
            let (Some(operand2), Some(operand1)) = (operand_stack.pop(), operand_stack.pop()) else {
                unreachable!()
            } ;

            let result = match operator {
                Operator::Plus => operand1 + operand2,
                Operator::Minus => operand1 - operand2,
                Operator::Multiply => operand1 * operand2,
                Operator::Divide => operand1 / operand2,
            };
            operand_stack.push(result);
        }

        return operand_stack.pop().unwrap();
    }
}

fn main() {
    fn into_test_case(origin: &[&str]) -> Vec<String> {
        origin
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
    }
    println!(
        "{}",
        Solution::eval_rpn(vec![
            "2".into(),
            "1".into(),
            "+".into(),
            "3".into(),
            "*".into()
        ])
    );
    println!(
        "{}",
        Solution::eval_rpn(vec![
            "4".into(),
            "13".into(),
            "5".into(),
            "/".into(),
            "+".into()
        ])
    );

    println!(
        "{}",
        Solution::eval_rpn(into_test_case(
            &["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
        ))
    );

    // "10","6","9","3","+","-11","*","/","*","17","+","5","+"
    // "10","6","9","3","+","-11","*","/","*","17","+"
    // "10","6","9","3","+","-11","*","/","*"
    // "10","6",    "12",   "-11","*","/","*"
    // 10 * (6 / (12 * -11))
    // 
}
