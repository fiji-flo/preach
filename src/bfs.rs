#[path = "graph.rs"]
mod graph;

pub fn bfs<F, GT: ::graph::GraphTraversal>(g: GT, s: usize, mapped_function: F)
    where F: Fn(usize) {
        mapped_function(s);
        for v in g.fwd_edges(s) {
            bfs(g, *v, mapped_function);
        }
    }
