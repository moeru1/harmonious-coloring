use anyhow::{anyhow, Error, Result};
use std::cell::RefCell;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Take;
use std::process;
use std::slice::Iter;

use harmonious_coloring::harmonious::HarmoniousColoring;
use harmonious_coloring::{graph, graph::Graph, Config};

fn main() -> Result<()> {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    const MAX_N: usize = 50;
    let results: RefCell<Vec<_>> = RefCell::new(Vec::new());
    graph::parse_file::<MAX_N, _>(&config.file_path, |graph| {
        let coloring = HarmoniousColoring::new().minimal_coloring(graph);
        results.borrow_mut().push(coloring.to_owned());
    })?;
    Ok(())
}
