use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{PathBuf, Path};
use std::cell::RefCell;
use std::ffi::OsStr;

use harmonious_coloring::{graph, graph::Graph, harmonious::HarmoniousColoring};

enum State {
    Title,
    HarmoniousNumber,
    Coloring,
    Diamater,
    EmptyLine,
}

fn file_out_parser(filename: &Path) -> Result<Vec<usize>> {
    let f = File::open(filename)?;
    let buffer = BufReader::new(f);
    let mut state = State::Title;
    let mut harmonious = Vec::new();
    for line in buffer.lines() {
        let line = line?;
        match state {
            State::Title => {
                state = State::HarmoniousNumber;
            }
            State::HarmoniousNumber => {
                state = State::Coloring;
                let mut split = line.split("=");
                split.next();
                let num_str = split.next()
                    .ok_or(anyhow!("Error: expected the following format.\nh = <usize>"))?;
                let h = num_str.trim().parse::<usize>().map_err(|_| anyhow!("{num_str} cannot be parsed to usize"))?;
                harmonious.push(h);
            }
            State::Coloring => {
                state = State::Diamater;
            }
            State::Diamater => {
                state = State::EmptyLine;
            }
            State::EmptyLine => {
                state = State::Title;
            }
        }
    }
    Ok(harmonious)
}

//https://stackoverflow.com/a/75880545
fn get_filenames(dir: &str, filter_ext: &OsStr) -> Result<Vec<PathBuf>> {
    let paths = std::fs::read_dir(dir)?
        // Filter out all those directory entries which couldn't be read
        .filter_map(|res| res.ok())
        // Map the directory entries to paths
        .map(|dir_entry| dir_entry.path())
        // Filter out all paths with extensions other than `csv`
        .filter_map(|path| {
            if path.extension().map_or(false, |ext| ext == filter_ext) {
                Some(path)
            } else {
                None
            }
        })
    .collect::<Vec<_>>();
    Ok(paths)
}

#[test]
fn test_all() -> Result<()> {
    let mut files_in = get_filenames("tests/test_files", OsStr::new("in"))?;
    files_in.sort();
    let mut files_out = get_filenames("tests/test_files", OsStr::new("out"))?;
    files_out.sort();
    //let mut ref_solutions = Vec::new();
    for (file_in, file_out) in files_in.iter().zip(&files_out) {
        println!("{:?} {:?}",file_in.file_name().unwrap(), file_out.file_name().unwrap());
        let reference_solution = file_out_parser(file_out.as_path())?;

        const MAX_N: usize = 50;
        let results: RefCell<Vec<_>> = RefCell::new(Vec::new());
        graph::parse_file::<MAX_N, _>(file_in.as_path(), |graph| {
            let h_object = HarmoniousColoring::new();
            let coloring = h_object.minimal_coloring(graph);
            let num_colors = coloring
                .iter()
                .map(|c| *c)
                .max()
                .expect("Error: Empty Coloring");
            let num_colors = num_colors + 1;
            results.borrow_mut().push(num_colors);
        })?;

        println!("Results: {:?}", *results.borrow());
        println!("Reference solution: {:?}", reference_solution);
        assert!(*results.borrow() == reference_solution);
    }
    Ok(())
}
