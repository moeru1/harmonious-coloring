use std::io::{BufReader, BufRead, Lines, Error};
use std::result::Result;
//use std::fs::File;
use std::iter::Filter;

struct ParseIterator<I>  {
    inner: I
    //i would like to do something like this
    // buffer: Filter<Lines<BufReader<File>>, fn filter_buffer(&Result<String, Error>) -> bool>, 
}

fn filter_buffer(line: &std::result::Result<String, std::io::Error>) -> bool {
    !line.as_ref().unwrap().is_empty()
}

impl<I: Iterator<Item = Result<String, Error>>> Iterator for ParseIterator<I> {
    type Item = i32;
    
    fn next(&mut self) -> Option<i32> {
        let line = match self.inner.next() {
            Some(line) => match line {
                Ok(line) => line,
                //ignore error just for the sake of making the minimal example simpler
                Err(_) => return None
            }
            None => return None,
        };
        let num = line.trim();
        let num = num.parse::<i32>().unwrap();
        
        Some(num)
    }
}

fn main() {
    let text = "1\n\n2\n3\n4\n\n\n5 \n61\n7\n8\n9\n10";
    let buffer = BufReader::new(text.as_bytes());
    let buffer_it: Filter<_, fn(&std::result::Result<String, std::io::Error>) -> bool> = buffer
            .lines()
            .filter(filter_buffer);

    
    let iterator = ParseIterator {
        inner: buffer_it,
    };

    for i in iterator {
        println!("{}", i);
    }

}
