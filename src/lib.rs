use std::path::PathBuf;
pub struct Config {
    pub file_path: PathBuf,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => return Err("Didn't get a file path"),
        };

        Ok(Config { file_path })
    }
}

pub mod graph {
    use anyhow::{anyhow, Result};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::iter::Take;
    use std::path::Path;
    use std::slice::Iter;
    use unordered_n_tuple::UnorderedPair;

    #[derive(Debug)]
    enum State {
        Header,
        Name,
        List,
    }

    pub struct Graph<const MAX_N: usize> {
        pub adj_list: [Vec<usize>; MAX_N],
        pub n: usize,
    }

    impl<const MAX_N: usize> Graph<MAX_N> {
        const ZERO_VEC: Vec<usize> = Vec::new();
        fn new(n: usize) -> Self {
            assert!(n <= MAX_N);
            return Self {
                adj_list: [Self::ZERO_VEC; MAX_N],
                n,
            };
        }

        pub fn iter(&self) -> Take<Iter<'_, Vec<usize>>> {
            self.adj_list.iter().take(self.n)
        }

        pub fn set_neighbors(&mut self, v: usize, neighbors: Vec<usize>) {
            self.adj_list[v] = neighbors;
        }

        pub fn get_neighbors(&self, v: usize) -> std::slice::Iter<'_, usize> {
            return self.adj_list[v].iter();
        }
    }

    pub fn parse_file<const MAX_N: usize, F>(file_path: &Path, func: F) -> Result<()>
    where
        F: Fn(Graph<MAX_N>) -> (),
    {
        let f = File::open(file_path)?;
        let buffer = BufReader::new(f);

        let mut graph: Option<Graph<MAX_N>> = None;
        let mut list_counter = 1;
        let mut state = State::Header;

        let mut n = 0;
        let mut num_graphs = 0;
        let mut line_num = 1;

        let mut buffer_it = buffer
            .lines()
            .filter(|line| !line.as_ref().unwrap().is_empty());

        for line in buffer_it {
            let line = line?;
            match state {
                State::Header => {
                    let tokens: Result<Vec<usize>, _> = line
                        .split_whitespace()
                        .take(2)
                        .map(|token| token.parse::<usize>())
                        .collect();

                    let tokens = match tokens {
                        Ok(t) => t,
                        Err(e) => {
                            return Err(anyhow!("Error in line {line_num}:\n{e}"));
                        }
                    };
                    if tokens.len() < 2 {
                        return Err(anyhow!(
                            "Expected format <num_vertices> <num_graphs> in line {line_num}"
                        ));
                    }
                    n = tokens[0];
                    num_graphs = tokens[1];
                    state = State::Name;
                }
                // Reading name of a graph
                State::Name => {
                    state = State::List;
                    graph = Some(Graph::new(n));
                    line_num += 1;
                    continue;
                }
                State::List => {
                    let tokens = line.split_whitespace();
                    let vertices: Result<Vec<usize>, _> =
                        tokens.map(|token| token.parse::<usize>()).collect();

                    let vertices = match vertices {
                        Ok(ver) => ver,
                        Err(e) => {
                            return Err(anyhow!(
                                "Error in line {line_num} in file {:?}:\n{e}",
                                file_path.file_name()
                            ));
                        }
                    };

                    let v = vertices[0] - 1;
                    let neighbors: Vec<usize> = vertices[1..].iter().map(|v| v - 1).collect();
                    //we know that at this point graph is initialized
                    graph.as_mut().unwrap().set_neighbors(v, neighbors);

                    if list_counter >= n {
                        func(graph.unwrap());
                        graph = None;
                        state = State::Name;
                        list_counter = 1;
                    } else {
                        list_counter += 1;
                    }
                }
            }
            line_num += 1;
        }
        Ok(())
    }
}

pub mod harmonious {
    use unordered_n_tuple::UnorderedPair;

    use crate::graph::Graph;
    use std::collections::HashSet;

    pub struct HarmoniousColoring {
        min_coloring: Vec<usize>,
        min_num_colors: usize,
        pairs_colors: HashSet<UnorderedPair<usize>>,
    }

    impl HarmoniousColoring {
        pub fn new() -> Self {
            // let current_coloring = Vec::new();
            let pairs_colors = HashSet::new();
            let min_coloring = Vec::new();
            //let current_num_colors = 0;
            let min_num_colors = std::usize::MAX;
            let visited = Vec::new();
            Self {
                min_coloring,
                pairs_colors,
                min_num_colors,
            }
        }

        pub fn visit_all(&mut self) {}

        pub fn minimal_coloring<const MAX_N: usize>(mut self, graph: Graph<MAX_N>) -> Vec<usize> {
            let mut current_coloring: Vec<Option<usize>> = vec![None; graph.n];
            let num_visited = 0;
            let num_colors = 0;
            println!("-----------Searching minimal coloring-----------------");
            for v in 0..graph.n {
                current_coloring[v] = Some(0);
                self.find_minimal(
                    &graph,
                    &mut current_coloring,
                    num_visited,
                    num_colors,
                    v,
                    true,
                );
                current_coloring[v] = None;
            }
            println!("min_coloring = {:?}", self.min_coloring);
            self.min_coloring
        }

        //inviariant: v is already marked by caller (parent)
        fn find_minimal<const MAX_N: usize>(
            &mut self,
            graph: &Graph<MAX_N>,
            current_coloring: &mut Vec<Option<usize>>,
            num_visited: usize,
            num_colors: usize,
            v: usize,
            is_harmonious: bool,
        ) {
           // println!("color[v] = {:?}", current_coloring[v]);
            let possible_colors = std::cmp::min(num_colors + 1, self.min_num_colors - 1);
            let parent_color =
                current_coloring[v].expect(&format!("Invalid inviariant: v={v} should have a color"));

            for &w in graph.get_neighbors(v) {
                for color in 0..possible_colors {
                    if !self.visited[w] {
                        self.visited[w] = true;
                        let new_num_colors = std::cmp::max(num_colors, color + 1);
                        let color_pair = UnorderedPair::from((parent_color, color));

                        current_coloring[w] = Some(color);
                        let current_harmonious = self.pairs_colors.insert(color_pair);
                        match self.test_coloring(
                            &graph,
                            current_coloring,
                            num_visited,
                            w,
                            is_harmonious,
                            ) {
                            Test::Reject => (),
                            Test::Accept => {
                                if num_colors < self.min_num_colors {
                                    self.min_num_colors = num_colors;
                                    let new_min: Option<Vec<usize>> =
                                        current_coloring.iter().map(|v| *v).collect();
                                    let new_min = new_min
                                        .expect("Error in backtracking, incomplete coloring");
                                    assert!(new_min.len() == graph.n);
                                    self.min_coloring = new_min.to_owned();
                                    println!("Accepted! with {num_colors} colors:\n{:?}", new_min);
                                }
                            }
                            Test::Continue => {
                                self.find_minimal(
                                    graph,
                                    current_coloring,
                                    num_visited + 1,
                                    new_num_colors,
                                    w,
                                    current_harmonious,
                                    );
                            }
                        }
                        //cleanup
                        current_coloring[v] = None;
                        self.pairs_colors.remove(&color_pair);
                        self.visited[w] = false;
                    }
                   // self.visited[w] = true;
                }
            }
        }

        fn test_coloring<const MAX_N: usize>(
            &self,
            graph: &Graph<MAX_N>,
            coloring: &[Option<usize>],
            num_visited: usize,
            current_vertex: usize,
            is_harmonious: bool,
        ) -> Test {
            let proper = self.is_proper(current_vertex, graph, coloring);
            let harmonious = self.is_harmonious(is_harmonious);
            if !harmonious {
                Test::Reject
            } else if harmonious && num_visited == graph.n {
                Test::Accept
            } else {
                Test::Continue
            }
        }

        fn is_proper<const MAX_N: usize>(
            &self,
            v: usize,
            graph: &Graph<MAX_N>,
            current_coloring: &[Option<usize>],
        ) -> bool {
            let color_v = current_coloring[v]
                .expect(&format!("Error in is_proper: Expected that v={v} had a color assigned"));
            let same_color = graph
                .get_neighbors(v)
                .filter_map(|&w| current_coloring[w])
                .find(|&color_w| color_w == color_v);
            same_color.is_none()
        }

        fn is_harmonious(&self, is_harmonous: bool) -> bool {
            is_harmonous
        }
    }

    enum Test {
        Reject,
        Accept,
        Continue,
    }
}
