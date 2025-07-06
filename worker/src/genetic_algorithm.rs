use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::graph::Graph;

pub struct Solution {
    labels: Vec<u8>,
    pub fitness: Option<usize>,
    modified: bool,
}

impl Clone for Solution {
    fn clone(&self) -> Self {
        Self {
            labels: self.labels.clone(),
            modified: true,
            fitness: self.fitness, // Option implementa a trait de copy
        }
    }
}

impl Solution {
    fn new(labels: Vec<u8>, fitness: Option<usize>) -> Self {
        Solution {
            labels,
            fitness,
            modified: true,
        }
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.fitness, other.fitness) {
            (Some(f1), Some(f2)) => Some(f1.cmp(&f2)),
            (None, Some(_)) => Some(Ordering::Less), // Considerar None como menor
            (Some(_), None) => Some(Ordering::Greater), // Considerar Some como maior
            (None, None) => Some(Ordering::Equal),   // Ambos são None
        }
    }
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap() // Aqui não precisa do unwrap_or
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for Solution {}

pub struct RomanDominationGA {
    graph: Graph,
    population_size: usize,
}

impl RomanDominationGA {
    pub fn new(graph: Graph, population_size: Option<usize>) -> Self {
        let population_size = population_size.unwrap_or(graph.get_num_vertices() / 2);
        RomanDominationGA {
            graph,
            population_size,
        }
    }

    fn generate_initial_population(&self) -> Vec<Solution> {
        let mut population = vec![];

        let h4_solution = self.generate_h4_solution();
        population.push(h4_solution);

        let h3_solution = self.generate_h3_solution();
        population.push(h3_solution);

        let h2_solution = self.generate_h2_solution();
        population.push(h2_solution);

        while population.len() < self.population_size {
            let h1_solution = self.generate_h1_solution();
            population.push(h1_solution);
        }

        population
    }

    fn generate_h2_solution(&self) -> Solution {
        let labels = self.graph.h2();
        let fitness = if labels.is_empty() {
            None
        } else {
            Some(labels.iter().map(|&x| x as usize).sum())
        };

        Solution::new(labels, fitness)
    }

    fn generate_h1_solution(&self) -> Solution {
        let labels = self.graph.h1();
        let fitness = if labels.is_empty() {
            None
        } else {
            Some(labels.iter().map(|&x| x as usize).sum())
        };

        Solution::new(labels, fitness)
    }

    fn generate_h3_solution(&self) -> Solution {
        let labels = self.graph.h3();
        let fitness = if labels.is_empty() {
            None
        } else {
            Some(labels.iter().map(|&x| x as usize).sum())
        };

        Solution::new(labels, fitness)
    }

    fn generate_h4_solution(&self) -> Solution {
        let labels = self.graph.h4();
        let fitness = if labels.is_empty() {
            None
        } else {
            Some(labels.iter().map(|&x| x as usize).sum())
        };

        Solution::new(labels, fitness)
    }

    fn evaluate_fitness(&self, solution: &mut Solution) -> usize {
        if solution.modified || solution.fitness.is_none() {
            if !self.is_feasible(solution) {
                self.make_feasible(solution);
            }

            let new_fitness = solution.labels.iter().map(|&x| x as usize).sum();
            solution.fitness = Some(new_fitness);
            solution.modified = false;
        }

        solution.fitness.unwrap()
    }

    fn is_feasible(&self, solution: &Solution) -> bool {
        for vertex in 0..self.graph.get_num_vertices() {
            if solution.labels[vertex] == 0
                && !self
                    .graph
                    .get_neighbors(vertex)
                    .iter()
                    .any(|&neighbor| solution.labels[neighbor] == 2)
            {
                return false;
            }
        }
        true
    }

    fn make_feasible(&self, solution: &mut Solution) {
        let mut was_modified = false;

        for vertex in 0..self.graph.get_num_vertices() {
            if solution.labels[vertex] == 0
                && !self
                    .graph
                    .get_neighbors(vertex)
                    .iter()
                    .any(|&neighbor| solution.labels[neighbor] == 2)
            {
                solution.labels[vertex] = 1;
                was_modified = true;
            }
        }

        if was_modified {
            solution.modified = true;
            solution.fitness = None;
        }
    }

    fn tournament_selection(
        &mut self,
        tournament_size: usize,
        population: &mut Vec<Solution>,
    ) -> Vec<Solution> {
        let mut selected = Vec::with_capacity(population.len());
        let mut rng = thread_rng();

        for solution in population.iter_mut() {
            if solution.modified || solution.fitness.is_none() {
                self.evaluate_fitness(solution);
            }
        }

        while selected.len() < population.len() {
            let tournament_indices: Vec<usize> = (0..population.len())
                .collect::<Vec<_>>()
                .choose_multiple(&mut rng, tournament_size)
                .cloned()
                .collect();

            let winner_index = tournament_indices
                .iter()
                .min_by_key(|&&idx| population[idx].fitness.unwrap())
                .unwrap();

            // Clone marca automaticamente como modified
            selected.push(population[*winner_index].clone());
        }

        selected
    }

    fn crossover(&self, parent_a: &Solution, parent_b: &Solution) -> Solution {
        let mut rng = thread_rng();
        let (r1, r2) = {
            let mut indices: Vec<usize> = (0..self.graph.get_num_vertices()).collect();
            indices.shuffle(&mut rng);
            (indices[0], indices[1])
        };

        let (r1, r2) = (r1.min(r2), r1.max(r2));
        let mut child_labels = parent_a.labels.clone();
        child_labels[r1..r2].copy_from_slice(&parent_b.labels[r1..r2]);

        Solution::new(child_labels, None)
    }

    pub fn run(
        &mut self,
        max_generations: usize,
        max_stagnant: usize,
        tournament_size: usize,
        crossover_probability: f32,
    ) -> Solution {
        let mut population: Vec<Solution> = self.generate_initial_population();

        for solution in &mut population {
            self.evaluate_fitness(solution);
        }

        let mut best_solution = population
            .iter()
            .min_by_key(|s| s.fitness.unwrap())
            .unwrap()
            .clone();

        let mut stagnant_generations = 0;
        let mut generation = 0;

        while generation < max_generations && stagnant_generations < max_stagnant {
            let intermediate_pop = self.tournament_selection(tournament_size, &mut population);
            let mut new_population = Vec::with_capacity(self.population_size);

            for i in (0..intermediate_pop.len()).step_by(2) {
                new_population.push(intermediate_pop[i].clone());

                if i + 1 < intermediate_pop.len() {
                    new_population.push(intermediate_pop[i + 1].clone());

                    if rand::random::<f32>() < crossover_probability {
                        let mut child =
                            self.crossover(&intermediate_pop[i], &intermediate_pop[i + 1]);
                        self.evaluate_fitness(&mut child);
                        if new_population.len() < self.population_size {
                            new_population.push(child);
                        }
                    }
                }

                if new_population.len() >= self.population_size {
                    new_population.truncate(self.population_size);
                    break;
                }
            }

            for solution in &mut new_population {
                if solution.modified || solution.fitness.is_none() {
                    self.evaluate_fitness(solution);
                }
            }

            population = new_population;

            let current_best = population
                .iter()
                .inspect(|s| {
                    if s.fitness.is_none() {
                        println!("Warning: Found solution without fitness!");
                    }
                })
                .min_by_key(|s| s.fitness.unwrap())
                .unwrap()
                .clone();

            if current_best.fitness < best_solution.fitness {
                best_solution = current_best;
                stagnant_generations = 0;
            } else {
                stagnant_generations += 1;
            }

            generation += 1;
        }

        best_solution
    }
}
