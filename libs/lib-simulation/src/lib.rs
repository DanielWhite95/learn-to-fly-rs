mod world;
mod food;
mod animal;
mod eye;

pub use self::{animal::*, food::*, world::*};
use nalgebra::geometry::Point2;
use nalgebra::{Rotation2, wrap, distance};
use rand::{rngs, Rng, RngCore};


pub struct Simulation {
    world: World
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore, num_animals: usize, num_food: usize) -> Self {
        Self {
            world: World::random(rng, num_animals, num_food)
        }
    }
    
    pub fn world(&self) -> &World {
        &self.world
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position.x = wrap(animal.position.x + animal.speed * animal.rotation.angle().cos(), 0.0, 1.0);
            animal.position.y = wrap(animal.position.y + animal.speed * animal.rotation.angle().sin(), 0.0, 1.0)
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.food {
                if distance(&animal.position, &food.position) < 0.01 {
                    food.position = rng.gen();
                    animal.score += 1;
                }
                
            }
        }
    }

    
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_movements();
        self.process_collisions(rng);
    }
}
