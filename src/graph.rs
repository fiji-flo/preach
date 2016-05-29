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
    pub fn new_with_edge_bwd(n: usize) -> Graph {
        Graph { e_fwd: Vec::new(), e_bwd: vec!(Vec::new(); n) }
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
    pub fn len(&self) -> usize {
        self.e_fwd.len()
    }
}

#[test]
fn simple_graph() {
    let mut g = Graph::new();
    g.push_edgelist_fwd(vec![2,1]);
    g.push_edgelist_fwd(vec![3,4]);
    g.push_edgelist_fwd(vec![]);
    g.push_edgelist_fwd(vec![]);
    g.push_edgelist_fwd(vec![]);
    g.push_edgelist_bwd(vec![]);
    g.push_edgelist_bwd(vec![0]);
    g.push_edgelist_bwd(vec![0]);
    g.push_edgelist_bwd(vec![1]);
    g.push_edgelist_bwd(vec![1]);
    g.sort_fwd();
    g.sort_bwd();
//    assert_eq!(g.e_fwd[0].iter().fold(true, |acc, &x| acc && x), true);
}
