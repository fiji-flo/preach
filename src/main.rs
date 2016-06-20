use std::env;
use std::vec::Vec;
mod graphreader;
mod graph;
mod dfs;
mod bfs;
mod mark;
mod constructiongraph;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg = args.get(1);
    if arg.is_some() {
        let g = graphreader::new_from_file(&(arg.unwrap())).unwrap();
        let mut marker = mark::TMarker::new(g.len());
        //dfs::dfs(&g, 0, &(Box::new(|x: usize| print!("{} ", x))), &mut marker);
        println!("doint a full dfs:");
        dfs::full_dfs(&g, &mut (Box::new(|x: usize| print!("{} ->", x))), &mut marker);
        marker.reset();
        println!("doint a full bfs:");
        bfs::full_bfs(&g, &mut (Box::new(|x: usize| print!("{} ->", x))), &mut marker);
        println!("done!");
        let cg = constructiongraph::ConstructionGraph::new_cg_from_file(&(arg.unwrap()));
    }
}
