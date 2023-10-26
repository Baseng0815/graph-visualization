use crate::binary_tree::BinaryTree;

mod binary_tree;

fn main() {
    let tree: BinaryTree<u32> = BinaryTree::new_random(10);
    tree.traverse_preorder(&|val| {
        println!("{}", val)
    });

    println!("{:?}", tree);
}
