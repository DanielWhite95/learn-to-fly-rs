use rand::{seq::SliceRandom, RngCore};

pub struct GeneticAlgorithm {}

pub trait Individual {
    fn fitness(&self) -> f32;
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
pub enum GeneticError {
    EmptyPopulation,
    SelectionError,
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

impl GeneticAlgorithm {
    pub fn evolve<T>(&self, population: &[T], rng: &mut dyn RngCore) -> Result<Vec<T>, GeneticError>
    where
        T: Individual,
    {
        if population.is_empty() {
            return Err(GeneticError::EmptyPopulation);
        };
        let selection_algorithm = RouletteWheelSelection {};
        let result = (0..population.len())
            .map(|_| {
                let parent_a = selection_algorithm
                    .select(rng, population)
                    .map_err(|e| eprintln!("Cannot select first parent"))
                    .ok();
                let parent_b = selection_algorithm
                    .select(rng, population)
                    .map_err(|e| eprintln!("Cannot select second parent"))
                    .ok();
                // TODO: crossover
                // TODO: mutation
                todo!()
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
    }

    impl Individual for TestIndividual {
        // add code here
        fn fitness(&self) -> f32 {
            self.fitness
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
                TestIndividual { fitness: 4.0 },
                TestIndividual { fitness: 1.0 },
                TestIndividual { fitness: 5.0 },
                TestIndividual { fitness: 10.0 },
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
