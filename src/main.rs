mod solution;
mod listnode;

use solution::Solution;
use listnode::ListNode;
fn main() {
    Solution::new();
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    /*    let mut input = String::new();
        println!("请输入一些文字：");
        match io::stdin().read_line(&mut input){
            Ok(_) => {
                println!("你输入的是：{}", input);
            }
            Err(error) => {
                println!("发生错误：{}", error);
            }
        }
        thread::sleep(Duration::from_secs(0));
    */
    println!("{:?}", Solution::is_palindrome(121));
    let mut l1 = ListNode::new(1);
    l1.next = Some(Box::new(ListNode::new(2)));
    l1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    l1.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut l2 = ListNode::new(1);
    l2.next = Some(Box::new(ListNode::new(2)));
    l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    l2.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut option = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    let mut result = String::new();
    while let  Some(mut node)= option{
        result.insert(result.len(), node.val.to_string().chars().nth(0).unwrap());
        println!("{}", result);
        option = node.next.take();
    }
}