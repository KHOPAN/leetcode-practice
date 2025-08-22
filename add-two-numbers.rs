impl Solution {
	pub fn add_two_numbers(mut first: Option<Box<ListNode>>, mut second: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut carry = 0;
		let mut root = Box::new(ListNode::new(0));
		let mut list = &mut root;

		loop {
			if !first.is_some() && !second.is_some() {
				break;
			}

			let mut sum = carry;

			if first.is_some() {
				let array = first.unwrap();
				sum += array.val;
				first = array.next;
			}

			if second.is_some() {
				let array = second.unwrap();
				sum += array.val;
				second = array.next;
			}

			carry = sum / 10;
			list.next = Some(Box::new(ListNode::new(sum % 10)));
			list = list.next.as_mut().unwrap();
		}

		if carry > 0 {
			list.next = Some(Box::new(ListNode::new(carry)));
		}

		Some(root.next.unwrap())
	}
}
