use knot::Knot;
use std::env;
mod colourings;
mod config;
mod knot;
mod parser;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::new(&args).unwrap();
    let knot = Knot::new(config);
    println!(
        "number of {:?}-colourings: {:?}",
        knot.p,
        knot.count_colourings()
    );
}
