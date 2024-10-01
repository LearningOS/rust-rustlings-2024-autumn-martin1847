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

    fn insert_from(root:&mut Box<TreeNode<T>>,value: T){
        if value == root.value {
            return;
        }else if value < root.value {
            // Self::insert_left(v,value);
            match root.left {
                Some(ref mut v) =>  Self::insert_from(v,value),
                None => root.left = Some(Box::new(TreeNode::new(value)))
            }
        }else{
            match root.right {
                Some(ref mut v) =>  Self::insert_from(v,value),
                None => root.right = Some(Box::new(TreeNode::new(value)))
            }
        }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root {
            Some(ref mut r) => Self::insert_from(r, value),
            None => self.root = Some(Box::new(TreeNode::new(value)))
        }
        // let mut start = &self.root;
        // while let Some(ref mut curr) = start {
        //     if value < curr.value {
        //         match curr.left {
        //             Some(ref l)=> {
        //                 if value < l.value {
        //                     start = &curr.left;
        //                     continue;
        //                 }
        //                 let mut me = Box::new(TreeNode::new(value));
        //                 me.left = curr.left;
        //                 curr.left = Some(me);
        //                 break;
        //             }
        //             None => {
        //                 curr.left = Some(Box::new(TreeNode::new(value)));
        //                 break;
        //             }
        //         }
        //         break;
        //     }

        //     match curr.right {
        //         Some(ref r)=> {
        //             if value > r.value {
        //                 start = &curr.left;
        //                 continue;
        //             }
        //             let mut me = Box::new(TreeNode::new(value));
        //             me.left = curr.left;
        //             curr.left = Some(me);
        //             break;
        //         }
        //         None => {
        //             curr.left = Some(Box::new(TreeNode::new(value)));
        //             break;
        //         }
        //     }

        // }
        //TODO
    }


    fn search_from(root:&Box<TreeNode<T>>,value: T) -> bool{
        if value == root.value {
            true
        }else if value < root.value {
            match root.left {
                Some(ref v) => Self::search_from(v, value),
                None => false
            }
        }else {
            match root.right {
                Some(ref v) => Self::search_from(v, value),
                None => false
            }
        }
        
    }
    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        // true
        match self.root {
            Some(ref r) => Self::search_from(r, value),
            None => false
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


