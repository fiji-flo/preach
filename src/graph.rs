pub trait GraphTraversal {
    fn bwd_edges(&self, v: usize) -> &Vec<usize>;
    fn fwd_edges(&self, v: usize) -> &Vec<usize>;
    fn all_bwd_edges(&self) -> &Vec<Vec<usize>>;
    fn all_fwd_edges(&self) -> &Vec<Vec<usize>>;
    fn sources(&self) -> Vec<usize>;
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
    fn all_bwd_edges(&self) -> &Vec<Vec<usize>> {
        &self.e_bwd
    }
    fn all_fwd_edges(&self) -> &Vec<Vec<usize>> {
        &self.e_fwd
    }
    fn sources(&self) -> Vec<usize> {
        self.e_bwd.iter().enumerate().filter_map(|(i, edges)| { if edges.len() == 0 { Some(i) } else { None } } ).collect::<Vec<usize>>()
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph { e_fwd: Vec::new(), e_bwd: Vec::new() }
    }
    pub fn new_with_edges(e_fwd: Vec<Vec<usize>>, e_bwd: Vec<Vec<usize>>) -> Graph {
        Graph { e_fwd: e_fwd, e_bwd: e_bwd }
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
    pub fn len(&self) -> usize {
        self.e_fwd.len()
    }
}

#[test]
fn simple_graph() {
}
