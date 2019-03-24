use std::cmp::PartialOrd;
use std::fmt::Debug;

pub struct Tree<T> {
    origin: Option<Box<Node<T>>>,
}

impl<T> Tree<T> {
    // Create a new, empty Tree
    pub fn new() -> Tree<T> {
        Tree {
            origin: None,
        }
    }

    // Add a new item
    pub fn add_item(&mut self, x: T) 
        where T: PartialOrd
    {
        if let Some(origin) = &mut self.origin {
            origin.add_item(x);
        } else {
            self.origin = Some(Node::new(x, 0, 0));
        } 
    }

    // Print a simple visualisation of the tree
    pub fn visualise(&self) 
        where T: Debug
    {
        if let Some(origin) = &self.origin {
            origin.print();
        }
    }

    // Clear the tree
    pub fn clear(&mut self) {
        self.origin = None; // Is this a good way of doing it? Will the other nodes be deallocated?
    }
}

struct Node<T> {
    layer: i32,
    index: i32,
    value: Option<T>,
    left_node: Option<Box<Node<T>>>,
    right_node: Option<Box<Node<T>>>,
}

impl<T> Node<T> {

    // Create a new node
    fn new(x: T, layer: i32, index: i32) -> Box<Node<T>> {
        Box::new(
            Node { 
                layer,
                index,
                value: Some(x),
                left_node: None,
                right_node: None,
            }
        )
    }

    // Add a new item to the node by either assigning its own value to x, creating new child node
    // or recursively call add_item on the already existing child node
    fn add_item(&mut self, x: T) 
        where T: PartialOrd
    {
        if let Some(value) = &self.value {
            if x < *value {
                if let Some(left_node) = &mut self.left_node { // Why do I need to borrow the value here using &mut, where would its ownership be moved to?
                    left_node.add_item(x);
                } else {
                    self.left_node = Some(Node::new(x, self.layer + 1, self.index - 1));
                }
            } else {
                if let Some(right_node) = &mut self.right_node {
                    right_node.add_item(x);
                } else {
                    self.right_node = Some(Node::new(x, self.layer + 1, self.index + 1));
                }
            }
        } else {
            self.value = Some(x);
        }
    }

    // Print a primitive version of the node, for debugging purposes
    fn print(&self) 
        where T: Debug
    {
        if let Some(value) = &self.value {
            println!("Value: {:?} | Layer: {} | Index: {}", *value, self.layer, self.index);
        }

        if let Some(node) = &self.left_node {
            node.print();
        }

        if let Some(node) = &self.right_node {
            node.print();
        }
    }
}