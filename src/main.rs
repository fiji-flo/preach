use std::env;
use std::vec::Vec;
mod graphreader;
mod graph;
mod bfs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1);
    if arg.is_some() {
        let g = graphreader::new_from_file(&(arg.unwrap()));
        assert!(g.is_some());
        bfs::bfs(g.unwrap(), 0, |x: usize| print!("{} ", x));
        println!("done!");
    }
}
