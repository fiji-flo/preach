use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn edges_from_file(filename: &String) -> Option<(usize, usize, Vec<(usize, usize)>)> {
    let f = match File::open(filename) {
        Ok(f) => f,
        Err(f) => panic!(f.to_string())
    };

    let mut reader = BufReader::new(f).lines().map(|l| { l.ok().expect("failed to read line") });
    reader.next().map(|l| { assert!(l.trim() == "graph_for_greach") });

    let n = reader.next().unwrap().split(" ").next().unwrap().parse::<u32>().unwrap() as usize;

    let edges = reader.map(|l| {
        let mut split = l.split(" ");
        let i: usize = split.next().unwrap().trim_matches(':').parse().unwrap();
        split.take_while(|c| { c.trim() != "#"}).map(|c| {
            (i, c.parse().ok().expect(&format!("failed to parse {}", c)))
        }).collect::<Vec<(usize, usize)>>()
    }).flat_map(|l| { l }).collect::<Vec<(usize, usize)>>();

    Some((n, edges.len(), edges))
}

pub fn new_from_file(filename: &String) -> Option<::graph::Graph> {
    let (n, m, edges) = edges_from_file(filename).unwrap_or((0, 0, Vec::new()));
    let mut e_fwd = vec!(Vec::new(); n);
    let mut e_bwd = vec!(Vec::new(); n);
    edges.iter().map(|&(s, t)| {
        e_fwd[s].push(t);
        e_bwd[t].push(s);
    }).collect::<Vec<_>>();
    let mut g = ::graph::Graph::new_with_edges(e_fwd, e_bwd);
    g.sort_bwd();
    println!("n: {}, m: {}", n, m);

    Some(g)
}
