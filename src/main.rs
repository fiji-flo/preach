use std::env;
use std::vec::Vec;
mod graphreader;
mod graph;
mod dfs;
mod bfs;
mod mark;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1);
    if arg.is_some() {
        let g = graphreader::new_from_file(&(arg.unwrap())).unwrap();
        let mut marker = mark::TMarker::new(g.len());
        dfs::dfs(&g, 0, &(|x: usize| print!("{} ", x)), &mut marker);
        bfs::bfs(&g, 0, &(|x: usize| print!("{} ", x)), &mut marker);
        println!("done!");
    }
}
