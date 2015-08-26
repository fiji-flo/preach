#[path = "graph.rs"]
mod graph;

use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn new_from_file(filename: &String) -> Option<::graph::Graph> {
    let n: usize;
    let mut m: usize = 0;
    let f = match File::open(filename) {
        Ok(f) => f,
        Err(f) => panic!(f.to_string())
    };
    let mut reader = BufReader::new(f);
    let buffer = &mut String::new();
    assert!(reader.read_line(buffer).is_ok());
    assert!(buffer.trim() == "graph_for_greach");
    buffer.clear();

    assert!(reader.read_line(buffer).is_ok());
    n = buffer.trim().parse::<usize>().unwrap();
    buffer.clear();

    let mut g = ::graph::Graph::new();

    for _ in 0..n {
        g.push_edgelist_bwd(Vec::new());
    }
    for i in 0..n {
        assert!(reader.read_line(buffer).is_ok());
        {
            let s = buffer.trim_left_matches(|c: char| c.is_numeric() || c == ':')
                .trim_right_matches(|c: char| c == '\n' || c == '#')
                .trim();
            let v: Vec<usize> = match s {
                "" => Vec::new(),
                _ => s.split(" ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect()
            };
            m += v.len();
            for s in &v {
                g.push_edge_bwd(*s, i);
            }

            g.push_edgelist_fwd(v);
        }
        buffer.clear();
    }

    g.sort_bwd();

    println!("n: {}, m: {}", n, m);

    Some(g)
}
