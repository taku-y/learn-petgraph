// use std::iter::{from_fn, FromIterator};
// use std::iter::FromIterator;
use std::fmt;
use petgraph::prelude::DiGraph;
use petgraph::algo::all_simple_paths;

struct MyNode {}

struct MyEdge {
    ix: u32
}

impl Default for MyNode {
    fn default() -> Self {
        Self {}
    }
}

impl fmt::Debug for MyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
         .field(&"node")
         .finish()
    }
}

impl Default for MyEdge {
    fn default() -> Self {
        Self { ix: 0 }
    }
}

fn main() {
    let g = DiGraph::<MyNode, MyEdge, _>::from_edges(&[
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 2),
        (1, 3),
        (2, 3),
        (2, 4),
        (3, 2),
        (3, 4),
        (4, 2),
        (4, 5),
        (5, 2),
        (5, 3)
    ]);
    let iter = all_simple_paths::<Vec<_>, _>(&g, 0u32.into(), 5u32.into(), 0, None);

    for p in iter {
        for n in &p {
            print!("{:?} ", g.node_weight(*n).unwrap());
        }
        println!();
    }
}
