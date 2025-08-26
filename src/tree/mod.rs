// 测试特性：二叉树数据结构、泛型、Box智能指针、递归遍历
// 语法要点：Option<Box<T>>、enum模式匹配、泛型<T>、递归函数
// 功能：实现泛型二叉树和三种遍历算法(前序、中序、后序)
// Use a generic type `T` to make the tree work with any data type.
// `Option<Box<Node<T>>>` is used to handle optional children and
// to manage heap allocation for the nodes.
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    // A simple constructor to create a new leaf node.
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

// An enum to represent the different traversal types.
enum TraversalType {
    InOrder,
    PreOrder,
    PostOrder,
}

// A function to perform the tree traversal. It takes a reference to the root
// node and the desired traversal type.
pub fn traverse<T: std::fmt::Display>(
    node: &Option<Box<Node<T>>>,
    traversal_type: &TraversalType,
) {
    if let Some(n) = node {
        match traversal_type {
            TraversalType::PreOrder => {
                // Visit the root first, then left, then right.
                println!("{}", n.value);
                traverse(&n.left, traversal_type);
                traverse(&n.right, traversal_type);
            }
            TraversalType::InOrder => {
                // Visit left, then the root, then right.
                traverse(&n.left, traversal_type);
                println!("{}", n.value);
                traverse(&n.right, traversal_type);
            }
            TraversalType::PostOrder => {
                // Visit left, then right, then the root.
                traverse(&n.left, traversal_type);
                traverse(&n.right, traversal_type);
                println!("{}", n.value);
            }
        }
    }
}

// The main function where we create and tranverse the tree.
pub fn main() {
    // Build a simple binary tree:
    //      1
    //     / \
    //    2   3
    //   / \
    //  4   5
    let mut root = Node::new(1);
    let mut node2 = Node::new(2);
    let node3 = Node::new(3);
    let node4 = Node::new(4);
    let node5 = Node::new(5);

    node2.left = Some(Box::new(node4));
    node2.right = Some(Box::new(node5));
    root.left = Some(Box::new(node2));
    root.right = Some(Box::new(node3));

    // Convert the root node into an Option for the `tranverse` function.
    let tree = Some(Box::new(root));

    // Perform and print the different traversals.
    println!("--- Pre-Order Traversal ---");
    traverse(&tree, &TraversalType::PreOrder);

    println!("\n--- In-Order Traversal ---");
    traverse(&tree, &TraversalType::InOrder);

    println!("\n--- Post-Order Traversal ---");
    traverse(&tree, &TraversalType::PostOrder);
}
