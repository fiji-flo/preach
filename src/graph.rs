//pub struct ContractionNode {
//    pub
//}
pub trait GraphTraversal {
    fn bwd_edges(&self, v: usize) -> &Vec<usize>;
    fn fwd_edges(&self, v: usize) -> &Vec<usize>;
}

pub struct Graph {
    e_fwd: Vec<Vec<usize>>,
    e_bwd: Vec<Vec<usize>>,
}

impl GraphTraversal for Graph {
    fn bwd_edges(&self, v: usize) -> &Vec<usize> {
        self.e_bwd.get(v).unwrap()
    }
    fn fwd_edges(&self, v: usize) -> &Vec<usize> {
        self.e_fwd.get(v).unwrap()
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph { e_fwd: Vec::new(), e_bwd: Vec::new() }
    }
    pub fn push_edge_fwd(&mut self, u: usize, v: usize) -> &Graph {
        self.e_fwd[u].push(v);
        self
    }
    pub fn push_edgelist_fwd(&mut self, v: Vec<usize>) -> &Graph {
        self.e_fwd.push(v);
        self
    }
    pub fn push_edge_bwd(&mut self, u: usize, v: usize) -> &Graph {
        self.e_bwd[u].push(v);
        self
    }
    pub fn push_edgelist_bwd(&mut self, v: Vec<usize>) -> &Graph {
        self.e_bwd.push(v);
        self
    }
    pub fn sort_fwd(&mut self) -> &Graph {
        for mut s in &mut self.e_fwd {
            s.sort();
        }
        self
    }
    pub fn sort_bwd(&mut self) -> &Graph {
        for mut s in &mut self.e_bwd {
            s.sort();
        }
        self
    }
}
