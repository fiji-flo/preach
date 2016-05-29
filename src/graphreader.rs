use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn new_from_file(filename: &String) -> Option<::graph::Graph> {
    let n: usize;
    let mut m: usize = 0;
    let f = match File::open(filename) {
        Ok(f) => f,
        Err(f) => panic!(f.to_string())
    };

    let mut reader = BufReader::new(f).lines().map(|l| { l.ok().expect("failed to read first line") });
    reader.next().map(|l| { assert!(l.trim() == "graph_for_greach") });

    n = reader.next().unwrap().split(" ").next().unwrap().parse::<u32>().unwrap() as usize;

    let mut g = ::graph::Graph::new_with_edge_bwd(n);

    reader.map(|l| {
        let mut split = l.split(" ");
        let i: usize = split.next().unwrap().trim_matches(':').parse().unwrap();
        let  edges = split.take_while(|c| { c.trim() != "#"}).map(|c| {
            c.parse().ok().expect(&format!("failed to parse {}", c))
        }).collect::<Vec<usize>>();
        m += edges.len();
        for s in &edges {
            g.push_edge_bwd(*s, i);
        }

        g.push_edgelist_fwd(edges);
    }).collect::<Vec<_>>();

    g.sort_bwd();

    println!("n: {}, m: {}", n, m);

    Some(g)
}
