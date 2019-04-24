#![allow(dead_code)]

use std::collections::vec_deque::VecDeque;

#[derive(Debug)]
struct GraphNode<'a, T> {
    value: T,
    children: Vec<&'a GraphNode<'a, T>>,
}

impl<'a, T> GraphNode<'a, T>
where
    T: PartialEq,
{
    fn add_node(&mut self, node: &'a GraphNode<T>) {
        self.children.push(node)
    }

    fn search(&self, value: T) -> Option<&GraphNode<T>> {
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            if node.value == value {
                return Some(node);
            }

            for child in node.children.clone() {
                queue.push_back(child)
            }
        }

        return None;
    }

    fn top_sort(&self) -> Vec<&T> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(self);

        while let Some(node) = queue.pop_front() {
            res.push(&node.value);

            for child in node.children.clone() {
                queue.push_back(child)
            }
        }

        return res;
    }
}

#[test]
fn graph_top_sort() {
    let mut graph = GraphNode {
        value: "One",
        children: Vec::new(),
    };

    let child = GraphNode {
        value: "And",
        children: Vec::new(),
    };
    graph.add_node(&child);

    let child = GraphNode {
        value: "Thanos",
        children: Vec::new(),
    };
    let child2 = GraphNode {
        value: "Only",
        children: vec![&child],
    };
    graph.add_node(&child2);

    dbg!(graph.top_sort());
}
