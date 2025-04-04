/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
		match &mut self.root {
			None => {
				self.root = Some(Box::new(TreeNode::new(value)));
			}
			Some(ref mut root) => {
				if root.value > value {
					match &mut root.left {
						Some(ref mut left) => left.insert(value),
						None => root.left = Some(Box::new(TreeNode::new(value)))
					}
				}
				else if (root.value < value){
					match &mut root.right {
						Some(ref mut right) => right.insert(value),
						None => root.right = Some(Box::new(TreeNode::new(value)))
					}
				}
			}
		}
		
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match &self.root {
			None => false,
			Some(root) => {
				if root.value == value {
					true
				}
				else if root.value > value  {
					match &root.left {
						Some(node) => {
							node.search(value)
						}
						None => false
					}
				}
				else {
					match &root.right {
						Some(node) => {
							node.search(value)
						}
						None => false
					}
				}
			}
		}
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
		if self.value > value {
			match &mut self.left {
				Some(ref mut left) => left.insert(value),
				None => self.left = Some(Box::new(TreeNode::new(value))),
			}
		}
		else {
			match &mut self.right {
				Some(ref mut right) => right.insert(value),
				None => self.right = Some(Box::new(TreeNode::new(value))),
			}
		}

    }
	fn search(&self, value: T) -> bool {
        //TODO
		match self.value.cmp(&value) {
			Ordering::Equal => {
				true
			}
			Ordering::Greater => {
				match &self.left {
					Some(node) => node.search(value),
					None => false
				}
			}
			Ordering::Less => {
				match &self.right {
					Some(node) => node.search(value),
					None => false
				}	
			}
		}
	}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


