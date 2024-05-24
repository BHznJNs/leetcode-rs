// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn push(node: &mut Option<Box<ListNode>>, val: i32) {
            let new_node = Some(Box::new(ListNode::new(val)));
            if node.is_none() {
                *node = new_node;
                return;
            }
            let mut target_node = node.as_mut().unwrap();
            loop {
                if target_node.next.is_none() {
                    target_node.next = new_node;
                    break;
                }
                let Some(sub_node) = &mut target_node.next else {
                    unreachable!()
                };
                target_node = sub_node;
            }
        }

        let mut result = None;
        let mut carry = false;
        let mut l1 = &l1;
        let mut l2 = &l2;

        loop {
            let mut sum = match (&l1, &l2) {
                (Some(node1), Some(node2)) => {
                    l1 = &node1.next;
                    l2 = &node2.next;
                    node1.val + node2.val
                }
                (Some(node), None) => {
                    l1 = &node.next;
                    node.val
                }
                (None, Some(node)) => {
                    l2 = &node.next;
                    node.val
                }
                (None, None) => {
                    if carry {
                        push(&mut result, 1);
                    }
                    break;
                },
            };

            if carry {
                carry = false;
                sum += 1;
            }
            if sum >= 10 {
                sum -= 10;
                carry = true;
            }
            push(&mut result, sum)
        }

        return result;
    }
}

fn main() {
    // 
}
