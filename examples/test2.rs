// use std::iter::{from_fn, FromIterator};
use std::iter::FromIterator;
use petgraph::prelude::DiGraph;
use petgraph::algo::all_simple_paths;

fn main() {
    let g = DiGraph::<i32, i32, _>::from_edges(&[
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
        println!("{:?}", p);
    }
}
