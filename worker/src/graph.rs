use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    usize,
};

use rand::seq::IteratorRandom;

#[derive(Clone)]
pub struct Graph {
    adjacency_list: Vec<Vec<usize>>, // Lista de adjacências com nós como strings
}

impl Graph {
    pub fn new(num_vertices: usize, edges: &[(usize, usize)]) -> Self {
        let mut adjacency_list = vec![vec![]; num_vertices];
        for &(u, v) in edges {
            adjacency_list[u].push(v);
            adjacency_list[v].push(u);
        }
        Graph { adjacency_list }
    }

    pub fn get_num_vertices(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn get_neighbors(&self, vertex: usize) -> &Vec<usize> {
        &self.adjacency_list[vertex]
    }

    pub fn get_vertex_degree(&self, vertex: usize) -> usize {
        self.adjacency_list[vertex].len()
    }

    pub fn add_vertex(&mut self, v: usize) {
        while self.adjacency_list.len() <= v {
            self.adjacency_list.push(vec![]);
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.add_vertex(u);
        self.add_vertex(v);

        self.adjacency_list[u].push(v);
        self.adjacency_list[v].push(u);
    }

    pub fn h1(&self) -> Vec<u8> {
        let mut f: Vec<u8> = vec![0; self.adjacency_list.len()];
        let mut unvisited: HashSet<usize> = (0..self.adjacency_list.len()).collect();
        let mut rng = rand::thread_rng();

        while !unvisited.is_empty() {
            let &u = unvisited.iter().choose(&mut rng).unwrap();
            f[u] = 2;
            unvisited.remove(&u);

            for &v in self.get_neighbors(u) {
                if unvisited.contains(&v) {
                    f[v] = 0;
                    unvisited.remove(&v);
                }
            }

            if unvisited.len() == 1 {
                let &last = unvisited.iter().next().unwrap();
                f[last] = 1;
                unvisited.remove(&last);
            }
        }
        f
    }

    pub fn h2(&self) -> Vec<u8> {
        let mut f: Vec<u8> = vec![0; self.adjacency_list.len()];
        let mut unvisited: Vec<usize> = (0..self.adjacency_list.len()).collect();

        unvisited.sort_by_key(|&vertex| std::cmp::Reverse(self.get_vertex_degree(vertex)));

        while !unvisited.is_empty() {
            let u = unvisited.remove(0);
            f[u] = 2;

            for &v in self.get_neighbors(u) {
                if let Some(pos) = unvisited.iter().position(|&x| x == v) {
                    f[v] = 0;
                    unvisited.remove(pos);
                }
            }

            if unvisited.len() == 1 {
                let last = unvisited[0];
                f[last] = 1;
                unvisited.clear();
            }
        }
        f
    }

    pub fn h3(&self) -> Vec<u8> {
        let mut f: Vec<u8> = vec![0; self.adjacency_list.len()];
        let mut unvisited: HashSet<usize> = (0..self.adjacency_list.len()).collect();

        while !unvisited.is_empty() {
            let mut max_degree = 0;
            let mut max_vertex = 0;
            for &vertex in &unvisited {
                let degree = self
                    .get_neighbors(vertex)
                    .iter()
                    .filter(|&&n| unvisited.contains(&n))
                    .count();
                if degree >= max_degree {
                    max_degree = degree;
                    max_vertex = vertex;
                }
            }

            f[max_vertex] = 2;
            unvisited.remove(&max_vertex);

            let neighbors: Vec<usize> = self
                .get_neighbors(max_vertex)
                .iter()
                .filter(|&&n| unvisited.contains(&n))
                .cloned()
                .collect();

            for neighbor in neighbors {
                f[neighbor] = 0;
                unvisited.remove(&neighbor);
            }

            if unvisited.len() == 1 {
                let last = *unvisited.iter().next().unwrap();
                f[last] = 1;
                unvisited.clear();
            }
        }
        f
    }

    pub fn h4(&self) -> Vec<u8> {
        let mut f: Vec<u8> = vec![0; self.adjacency_list.len()];
        let mut unvisited: HashSet<usize> = (0..self.adjacency_list.len()).collect();

        while !unvisited.is_empty() {
            let mut max_degree = 0;
            let mut max_vertex = 0;
            for &vertex in &unvisited {
                let degree = self
                    .get_neighbors(vertex)
                    .iter()
                    .filter(|&&n| unvisited.contains(&n))
                    .count();
                if degree >= max_degree {
                    max_degree = degree;
                    max_vertex = vertex;
                }
            }

            f[max_vertex] = 2;

            let neighbors: Vec<usize> = self
                .get_neighbors(max_vertex)
                .iter()
                .filter(|&&n| unvisited.contains(&n))
                .cloned()
                .collect();

            for neighbor in neighbors {
                f[neighbor] = 0;
                unvisited.remove(&neighbor);
            }

            unvisited.remove(&max_vertex);

            let isolated: Vec<usize> = unvisited
                .iter()
                .filter(|&&vertex| {
                    self.get_neighbors(vertex)
                        .iter()
                        .filter(|&&n| unvisited.contains(&n))
                        .count()
                        == 0
                })
                .cloned()
                .collect();

            for vertex in isolated {
                f[vertex] = 1;
                unvisited.remove(&vertex);
            }

            if unvisited.len() == 1 {
                let last = *unvisited.iter().next().unwrap();
                f[last] = 1;
                unvisited.clear();
            }
        }
        f
    }

    pub fn from_file(file_path: String) -> io::Result<Self> {
        let file = File::open(&file_path)?;
        let reader = io::BufReader::new(file);
        let mut g = Graph::new(0, &[]);

        for line in reader.lines() {
            let line = line?;
            let vertices: Vec<usize> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();

            if vertices.len() == 2 {
                let (u, v) = (vertices[0], vertices[1]);
                g.add_vertex(u);
                g.add_vertex(v);
                if u != v {
                    g.add_edge(u, v);
                }
            }
        }

        Ok(g)
    }

    pub fn get_graph_size(&self) -> usize {
        let mut edge_count = 0;
        for v in &self.adjacency_list {
            edge_count += v.len();
        }
        edge_count / 2
    }
}
