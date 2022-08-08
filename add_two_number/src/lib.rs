use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

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
// pub fn add_two_numbers(
//     l1: Option<Box<ListNode>>,
//     l2: Option<Box<ListNode>>,
// ) -> Option<Box<ListNode>> {
//     let mut result: Option<ListNode> = None;

//     let mut vec_dequeue = VecDeque::new();
//     let mut carry = 1;

//     let mut node_l1 = l1;
//     let mut node_l2 = l2;

//     loop {
//         let mut sum = 0;

//         if node_l1.is_none() && node_l2.is_none() {
//             break;
//         }

//         if node_l1.is_some() {
//             sum = sum + node_l1.as_ref().unwrap().val;
//         }

//         if node_l2.is_some() {
//             sum = sum + node_l2.as_ref().unwrap().val;
//         }

//         if carry > 0 {
//             sum = sum + carry;
//             carry = 0;
//         }

//         if sum > 10 {
//             carry = 1;
//             vec_dequeue.push_front(sum - 10);
//         } else {
//             carry = 0;
//             vec_dequeue.push_front(sum);
//         }

//         if node_l1.as_ref().unwrap().next.is_some() {
//             node_l1 = node_l1.unwrap().next;
//         }

//         if node_l2.as_ref().unwrap().next.is_some() {
//             node_l2 = node_l2.unwrap().next;
//         }
//     }

//     if vec_dequeue.len() == 0 {
//         return None;
//     }

//     for item in vec_dequeue {
//         if result.is_none() {
//             result = Some(ListNode::new(item));
//         } else {
//             let mut list_node = ListNode::new(item);
//             list_node.next = Some(Box::new(result.unwrap()));
//             result = Some(list_node);
//         }
//     }

//     Some(Box::new(result.unwrap()))
// }

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    crate::add(l1, l2, 0);
    todo!()
}

pub fn add(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() {
        return None;
    }

    let (val1, next1) = extract(l1);
    let (val2, next2) = extract(l2);

    let sum = val1 + val2 + carry;

    let mut node = ListNode::new(sum % 10);
    node.next = add(next1, next2, sum / 10);

    Some(Box::new(node))
}

pub fn extract(l: Option<Box<ListNode>>) -> (i32, Option<Box<ListNode>>) {
    match l {
        None => (0, None),
        Some(box_val) => (box_val.val, box_val.next),
    }
}
