use nalgebra::geometry::Point2;
use nalgebra::{Rotation2, wrap, distance};
use rand::{rngs, Rng, RngCore};

use crate::animal::*;
use crate::food::*;

#[derive(Debug)]
pub struct World {
    pub(crate) animals: Vec<Animal>,
    pub(crate) food: Vec<Food>
}


impl World {
    pub fn random(rng: &mut dyn RngCore, num_animals: usize, num_food: usize) -> Self {
        Self {
            animals: (0..num_animals).map(|_| Animal::random(rng)).collect(),
            food: (0..num_food).map(|_| Food::random(rng)).collect()
        }
    }
    
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn food(&self) -> &[Food] {
        &self.food
    }
    

} 
