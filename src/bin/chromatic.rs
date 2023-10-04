use anyhow::Result;
use harmonious_coloring::graph::ParseIterator;
use std::cell::RefCell;
use std::env;
use std::process;
use std::io::{BufReader, BufRead};

use harmonious_coloring::harmonious::HarmoniousColoring;
use harmonious_coloring::{graph, Config};

fn main() -> Result<()> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    const MAX_N: usize = 50;
    let results: RefCell<Vec<_>> = RefCell::new(Vec::new());
    let file = std::fs::File::open(&config.file_path)?;
    let buffer = BufReader::new(file);

    let it = ParseIterator::<MAX_N, _>::new(buffer)?;

    for g in it {
        let g = g?;
        let coloring = HarmoniousColoring::new().minimal_coloring(g);
        results.borrow_mut().push(coloring.to_owned());
    }

    print_results(&results.borrow());

    Ok(())
}

fn print_results(results: &Vec<Vec<usize>>) {
    for (i, coloring) in results.iter().enumerate() {
        println!("Graph {}: ", i + 1);
        let num_colors = coloring.iter().max().expect("Error: Empty Coloring") + 1;

        println!("h = {}", num_colors);

        for color in coloring.iter() {
            print!("{} ", color);
        }

        println!("");
        println!("diameter = {}", -1);
        println!("");
    }
}
