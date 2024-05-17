use crate::listnode::ListNode;

pub struct Solution;
impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                return vec![*map.get(&complement).unwrap(), i as i32];
            }
            map.insert(num, i as i32);
        }
        vec![]
    }


    pub fn is_palindrome(x: i32) -> bool {
        if x<0 { return  false }
        x.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
            == x
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut head = ListNode::new(0, );
        let mut tail = &mut head;
        let mut carry = 0;
        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }
            let node = ListNode::new(sum%10, );
            tail.next = Some(Box::new(node));
            tail = tail.next.as_mut().unwrap();
            carry = sum/10;
        }
        if carry > 0 {
            tail.next = Some(Box::new(ListNode::new(carry, )));
        }
        head.next
    }
    pub fn new()->Solution{
        Solution
    }

}