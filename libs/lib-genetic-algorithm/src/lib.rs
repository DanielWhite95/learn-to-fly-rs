use std::{ops::Index, vec::IntoIter};

use rand::{seq::SliceRandom, Rng, RngCore};


pub trait Individual {
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Chromosome;
    fn create(chromosome: Chromosome) -> Self;
}

pub trait Selection {
    fn select<'a, T>(
        &self,
        rng: &mut dyn RngCore,
        population: &'a [T],
    ) -> Result<&'a T, GeneticError>
    where
        T: Individual;
}


#[derive(Debug)]
pub struct Chromosome {
    genes: Vec<f32>
}

impl Chromosome {
    // add code here
    pub fn genes(&self) -> &[f32] {
        &self.genes
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

impl Index<usize> for Chromosome {
    // Access directly a specific gene by index as in vectors
    type Output = f32;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
    
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = IntoIter<f32>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }

}

impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }

}


pub trait Crossover {
    fn mix_parents(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Result<Chromosome, GeneticError>;
}

pub struct UniformCrossover;

impl Crossover for UniformCrossover {
    fn mix_parents(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Result<Chromosome, GeneticError> {
        if parent_a.genes().len() != parent_b.genes().len() {
            return Err(GeneticError::CrossoverError);
        }
        Ok(Chromosome { 
            genes: parent_a.iter().zip(parent_b.iter()).map(|(&gene_a, &gene_b)| if rng.gen_bool(0.5) { gene_a } else { gene_b}).collect()
        })
    }
}


pub trait Mutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

pub struct GaussianMutation {
    chance: f32,
    coeff: f32
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Result<Self, GeneticError> {
        if chance < 0.0 || chance > 1.0 {
            return Err(GeneticError::ValidationError)
        }
        Ok(
            Self {
                chance,
                coeff
            }
        )
    }
}

impl Mutation for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            
            if rng.gen_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.gen::<f32>(); 
            }
        }
    }
}

#[derive(Debug)]
pub enum GeneticError {
    EmptyPopulation,
    SelectionError,
    CrossoverError,
    ValidationError
}

pub struct RouletteWheelSelection {}

impl Selection for RouletteWheelSelection {
    fn select<'a, T>(
        &self,
        rng: &mut dyn RngCore,
        population: &'a [T],
    ) -> Result<&'a T, GeneticError>
    where
        T: Individual,
    {
        population
            .choose_weighted(rng, |i| i.fitness())
            .map_err(|e| {
                eprintln!("Cannot select individual {:?}", e);
                GeneticError::SelectionError
            })
    }
}

pub struct GeneticAlgorithm<S: Selection, C: Crossover, M: Mutation>  {
    selection_method: S,
    crossover_algorithm: C,
    mutation_algorithm: M
}

impl<S: Selection, C: Crossover, M: Mutation> GeneticAlgorithm<S, C, M> {
    pub fn evolve<T>(&self, population: &[T], rng: &mut dyn RngCore) -> Result<Vec<T>, GeneticError>
    where
        T: Individual,
    {
        if population.is_empty() {
            return Err(GeneticError::EmptyPopulation);
        };
        let result = (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method
                    .select(rng, population)
                    .map_err(|e| eprintln!("Cannot select first parent"))
                    .ok();
                let parent_b = self.selection_method
                    .select(rng, population)
                    .map_err(|e| eprintln!("Cannot select second parent"))
                    .ok();
                let mut new_chromosome = self.crossover_algorithm.mix_parents(rng, parent_a.unwrap().chromosome(), parent_b.unwrap().chromosome()).expect("Cannot mix individuals!");    
                self.mutation_algorithm.mutate(rng, &mut new_chromosome);
                T::create(new_chromosome)
            })
            .collect();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[derive(Debug)]
    struct TestIndividual {
        fitness: f32,
        genes: Vec<f32>
    }

    impl Individual for TestIndividual {
        // add code here
        fn fitness(&self) -> f32 {
            self.fitness
        }
        fn chromosome(&self) -> &Chromosome {
            todo!()
        }
        
        fn create(chromosome: Chromosome) -> Self {
            Self {
                fitness: 0.0,
                genes: chromosome.genes
            }
        }
    }

    mod selection {
        use super::*;
        use std::collections::BTreeMap;
        use std::iter::FromIterator;

        #[test]
        fn roulette_wheel_selection() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let population = vec![
                TestIndividual { fitness: 4.0 , genes: vec![] },
                TestIndividual { fitness: 1.0 , genes: vec![] },
                TestIndividual { fitness: 5.0 , genes: vec![] },
                TestIndividual { fitness: 10.0, genes: vec![] },
            ];
            let mut actual_histogram: BTreeMap<i32, i32> = BTreeMap::new();

            let selection_method = RouletteWheelSelection {};
            _ = (0..100).map(|_| {
                let fitness = selection_method.select(&mut rng, &population).expect("Failed to apply selection in tests").fitness();
                *actual_histogram.entry(fitness as i32).or_insert(0) += 1;
                fitness
            }).collect::<Vec<_>>();

            let expected_histogram = BTreeMap::from_iter([
                (10,47),
                (5,30),
                (4,18),
                (1,5)
            ]);
            assert_eq!(actual_histogram, expected_histogram);
        }
    }
    // fn it_works() {
    //     todo!()
    // }
}
