use rand::{thread_rng, Rng, prelude::Distribution, distributions::Standard};

#[derive(Debug)]
pub struct BinaryTree<T>
where Standard: Distribution<T> {
    root: Option<TreeNode<T>>
}

#[derive(Debug)]
pub struct TreeNode<T>
where Standard: Distribution<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>
}

impl<T> TreeNode<T>
where Standard: Distribution<T> {
    pub fn new(val: T) -> TreeNode<T> {
        TreeNode { val, left: None, right: None }
    }

    pub fn new_random(node_count: u32) -> TreeNode<T> {
        let mut node = TreeNode::new(thread_rng().gen::<T>());

        if node_count > 0 {
            let ratio: f64 = thread_rng().gen_range(0.0..1.0);
            let count_left = (ratio * (node_count - 1) as f64) as u32;
            let count_right = node_count - count_left - 1;

            node.left = Some(Box::new(TreeNode::new_random(count_left)));

            if count_right > 0 {
                node.right = Some(Box::new(TreeNode::new_random(count_right)));
            }
        }

        node
    }

    pub fn traverse_preorder<F>(&self, func: &F)
        where F: Fn(&T) -> ()
        {
            func(&self.val);
            if let Some(left) = &self.left {
                left.traverse_preorder(func);
            }

            if let Some(right) = &self.right {
                right.traverse_preorder(func);
            }
        }
}

impl<T> BinaryTree<T>
where Standard: Distribution<T> {
    pub fn new() -> BinaryTree<T> {
        BinaryTree {
            root: None
        }
    }

    pub fn new_random(node_count: u32) -> BinaryTree<T> {
        BinaryTree { root: Some(TreeNode::new_random(node_count)) }
    }

    pub fn traverse_preorder<F>(&self, func: &F)
        where F: Fn(&T) -> ()
        {
            if let Some(root) = &self.root {
                root.traverse_preorder(func);
            }
        }
}
