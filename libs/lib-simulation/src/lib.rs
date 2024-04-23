use nalgebra::geometry::Point2;
use nalgebra::Rotation2;
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
    
    pub fn step(&mut self) {
        for animal in self.world.animals_mut() {
            animal.position.x += animal.speed * animal.rotation.angle().cos();
            animal.position.y += animal.speed * animal.rotation.angle().sin();
        }
    }
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>
}


impl World {
    fn random(rng: &mut dyn RngCore, num_animals: usize, num_food: usize) -> Self {
        Self {
            animals: (0..num_animals).map(|_| Animal::random(rng)).collect(),
            food: (0..num_food).map(|_| Food::random(rng)).collect()
        }
    }
    
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn animals_mut(&mut self) -> &mut [Animal] {
        &mut self.animals
    }
    
    pub fn food(&self) -> &[Food] {
        &self.food
    }
} 


#[derive(Debug)]
pub struct Animal {
    position: Point2<f32>,
    rotation: Rotation2<f32>,
    speed: f32
}

impl Animal {
    fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }
}

#[derive(Debug)]
pub struct Food {
    position: Point2<f32>
}


impl Food {
    fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen()
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}

