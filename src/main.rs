use std::env;
use std::vec::Vec;
mod graphreader;
mod graph;
mod dfs;
mod bfs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1);
    if arg.is_some() {
        let g = graphreader::new_from_file(&(arg.unwrap())).unwrap();
        dfs::dfs(&g, 0, &(|x: usize| print!("{} ", x)));
        bfs::bfs(&g, 0, &(|x: usize| print!("{} ", x)));
        println!("done!");
    }
}
