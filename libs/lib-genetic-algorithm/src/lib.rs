use rand::{distributions::WeightedError, seq::SliceRandom, RngCore};

pub struct GeneticAlgorithm {
}

pub trait Individual {
    fn fitness(&self) -> f32;
}




pub trait Selection {
    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> Result<&'a T, GeneticError>
    where
        T: Individual;
}


#[derive(Debug)]
pub enum GeneticError {
    EmptyPopulation,
    SelectionError
}

pub struct RouletteWheelSelection {}

impl Selection for RouletteWheelSelection {

    fn select<'a, T>(&self, rng: &mut dyn RngCore, population: &'a [T]) -> Result<&'a T, GeneticError>
        where
            T: Individual {
        
        population.choose_weighted(rng, |i| i.fitness()).map_err(|e| {
            eprintln!("Cannot select individual {:?}", e);
            GeneticError::SelectionError
        })
    }
}

impl GeneticAlgorithm {
    pub fn evolve<T>(&self, population: &[T], rng: &mut dyn RngCore) -> Result<Vec<T>, GeneticError>
    where 
        T: Individual
    {
        if population.is_empty() {
           return Err(GeneticError::EmptyPopulation) 
        };
        let selection_algorithm = RouletteWheelSelection{};
        let result:  = (0..population.len())
            .map(|_| {
                    let parent_a = selection_algorithm.select(rng, population).map_err(|e| eprintln!("Cannot select first parent")).ok();
                    let parent_b = selection_algorithm.select(rng, population).map_err(|e| eprintln!("Cannot select second parent")).ok();
                    // TODO: crossover
                    // TODO: mutation
                    Some(parent_a.unwrap())
            }
        ).collect();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
