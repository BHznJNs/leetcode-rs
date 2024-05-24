// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;
impl Solution {
    // pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // use std::mem::replace;

        // if let Some(head_node) = head {
        //     let mut current_node = head_node;
        //     while let Some(mut next_node) = current_node.next.take() {
        //         if let Some(next_next_node) = next_node.next.take() {
        //             current_node.next = Some(next_next_node);
        //             next_node.next = Some(current_node);
        //             current_node = next_next_node;
        //         }
        //     }
        //     todo!()
        // } else {
        //     return None;
        // }
    // }
}

fn main() {
    // 
}