/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
#[derive(PartialEq)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Clone + std::cmp::PartialEq> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + std::cmp::PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	fn reverse_list(start: Option<NonNull<Node<T>>>, end: Option<NonNull<Node<T>>>) {
		match (start, end) {
				(Some(ptr1), Some(ptr2)) => {
					
						let mut x = unsafe{&mut (*ptr1.as_ptr()).val};
						let mut y = unsafe{&mut (*ptr2.as_ptr()).val};
						println!("Reverse!!");				
						let tmp = x.clone();
						*x = y.clone();
						*y = tmp;
						//start = unsafe{(*ptr1.as_ptr()).next};
						//end = unsafe{(*ptr2.as_ptr()).prev};
unsafe {
						if (*ptr1.as_ptr()).val == (*ptr2.as_ptr()).val {
							return;	
						}
						if (*ptr1.as_ptr()).next == end {
							return;
						}
						}
						Self::reverse_list(unsafe{(*ptr1.as_ptr()).next}, unsafe{(*ptr2.as_ptr()).prev});
				
				}
				(None, Some(p)) => {println!("Start is none");}
				(Some(p), None) => {println!("End is none");}
				_ => {println!("Something is wrong!!");}
			}
	}
	pub fn reverse(&mut self){
		// TODO
		Self::reverse_list(self.start, self.end);	
		/*loop {
			match (start, end) {
				(Some(ptr1), Some(ptr2)) => {
					unsafe {
						if (*ptr1.as_ptr()) == (*ptr2.as_ptr()) {
							break;
						}
						let mut x = unsafe{&mut (*ptr1.as_ptr()).val};
						let mut y = unsafe{&mut (*ptr2.as_ptr()).val};
						
						let tmp = x.clone();
						*x = y.clone();
						*y = tmp;

						start = unsafe{(*ptr1.as_ptr()).next};
						end = unsafe{(*ptr2.as_ptr()).prev};
					}

				}
				_ => {println!("Something wrong!!");}
			}

		}
		*/
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}
