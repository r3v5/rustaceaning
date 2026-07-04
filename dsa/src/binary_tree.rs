use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct TreeNode<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Option<Rc<RefCell<TreeNode<T>>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        })))
    }
}

pub fn preorder<T: Clone>(root: &Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    let mut result = Vec::new();
    fn dfs<T: Clone>(node: &Option<Rc<RefCell<TreeNode<T>>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            let n = n.borrow();
            result.push(n.value.clone());
            dfs(&n.left, result);
            dfs(&n.right, result);
        }
    }
    dfs(root, &mut result);
    result
}

pub fn inorder<T: Clone>(root: &Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    let mut result = Vec::new();
    fn dfs<T: Clone>(node: &Option<Rc<RefCell<TreeNode<T>>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            let n = n.borrow();
            dfs(&n.left, result);
            result.push(n.value.clone());
            dfs(&n.right, result);
        }
    }
    dfs(root, &mut result);
    result
}

pub fn postorder<T: Clone>(root: &Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<T> {
    let mut result = Vec::new();
    fn dfs<T: Clone>(node: &Option<Rc<RefCell<TreeNode<T>>>>, result: &mut Vec<T>) {
        if let Some(n) = node {
            let n = n.borrow();
            dfs(&n.left, result);
            dfs(&n.right, result);
            result.push(n.value.clone());
        }
    }
    dfs(root, &mut result);
    result
}

pub fn bfs<T: Clone>(root: &Option<Rc<RefCell<TreeNode<T>>>>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    let mut queue = VecDeque::new();

    if let Some(node) = root {
        queue.push_back(Rc::clone(node));
    }

    while !queue.is_empty() {
        let queue_size = queue.len();
        let mut curr_level = Vec::new();

        for _ in 0..queue_size {
            let curr = queue.pop_front().unwrap();
            let n = curr.borrow();
            curr_level.push(n.value.clone());

            if let Some(left) = &n.left {
                queue.push_back(Rc::clone(left));
            }
            if let Some(right) = &n.right {
                queue.push_back(Rc::clone(right));
            }
        }

        result.push(curr_level);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree() -> Option<Rc<RefCell<TreeNode<i32>>>> {
        //        1
        //       / \
        //      2   3
        //     / \
        //    4   5
        let root = TreeNode::new(1);
        let left = TreeNode::new(2);
        let right = TreeNode::new(3);
        let left_left = TreeNode::new(4);
        let left_right = TreeNode::new(5);

        left.as_ref().unwrap().borrow_mut().left = left_left;
        left.as_ref().unwrap().borrow_mut().right = left_right;
        root.as_ref().unwrap().borrow_mut().left = left;
        root.as_ref().unwrap().borrow_mut().right = right;
        root
    }

    #[test]
    fn test_preorder() {
        let root = build_tree();
        assert_eq!(preorder(&root), vec![1, 2, 4, 5, 3]);
    }

    #[test]
    fn test_inorder() {
        let root = build_tree();
        assert_eq!(inorder(&root), vec![4, 2, 5, 1, 3]);
    }

    #[test]
    fn test_postorder() {
        let root = build_tree();
        assert_eq!(postorder(&root), vec![4, 5, 2, 3, 1]);
    }

    #[test]
    fn empty_tree() {
        let root: Option<Rc<RefCell<TreeNode<i32>>>> = None;
        assert_eq!(preorder(&root), vec![]);
        assert_eq!(inorder(&root), vec![]);
        assert_eq!(postorder(&root), vec![]);
    }

    #[test]
    fn single_node() {
        let root = TreeNode::new(42);
        assert_eq!(preorder(&root), vec![42]);
        assert_eq!(inorder(&root), vec![42]);
        assert_eq!(postorder(&root), vec![42]);
        assert_eq!(bfs(&root), vec![vec![42]]);
    }

    #[test]
    fn test_bfs() {
        let root = build_tree();
        assert_eq!(bfs(&root), vec![vec![1], vec![2, 3], vec![4, 5]]);
    }

    #[test]
    fn bfs_empty() {
        let root: Option<Rc<RefCell<TreeNode<i32>>>> = None;
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(bfs(&root), empty);
    }
}
