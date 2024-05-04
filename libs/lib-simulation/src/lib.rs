mod world;
mod food;
mod animal;
mod eye;

pub use self::{animal::*, food::*, world::*};
use nalgebra::geometry::Point2;
use genetic_algorithm::{GeneticAlgorithm, Chromosome, Individual, RouletteWheelSelection, UniformCrossover, GaussianMutation};
use nalgebra::{Rotation2, wrap, distance};
use rand::{rngs, Rng, RngCore};
use std::f32::consts::FRAC_PI_4;

/// Minimum speed of a bird.
///
/// Keeping it above zero prevents birds from getting stuck in one place.
const SPEED_MIN: f32 = 0.001;

/// Maximum speed of a bird.
///
/// Keeping it "sane" prevents birds from accelerating up to infinity,
/// which makes the simulation... unrealistic :-)
const SPEED_MAX: f32 = 0.005;

/// Speed acceleration; determines how much the brain can affect bird's
/// speed during one step.
///
/// Assuming our bird is currently flying with speed=0.5, when the brain
/// yells "stop flying!", a SPEED_ACCEL of:
///
/// - 0.1 = makes it take 5 steps ("5 seconds") for the bird to actually
///         slow down to SPEED_MIN,
///
/// - 0.5 = makes it take 1 step for the bird to slow down to SPEED_MIN.
///
/// This improves simulation faithfulness, because - as in real life -
/// it's not possible to increase speed from 1km/h to 50km/h in one
/// instant, even if your brain very much wants to.
const SPEED_ACCEL: f32 = 0.2;

/// Ditto, but for rotation:
///
/// - 2 * PI = it takes one step for the bird to do a 360° rotation,
/// - PI = it takes two steps for the bird to do a 360° rotation,
///
/// I've chosen PI/2, because - as our motto goes - this value seems
/// to play nice.
const ROTATION_ACCEL: f32 = FRAC_PI_4;


pub struct Simulation  {
    world: World,
    evolution_algorithm: GeneticAlgorithm<RouletteWheelSelection, UniformCrossover, GaussianMutation>
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore, num_animals: usize, num_food: usize, mut_chance: f32, mut_coeff: f32) -> Self {
        Self {
            world: World::random(rng, num_animals, num_food),
            evolution_algorithm: GeneticAlgorithm::new(
                RouletteWheelSelection {},
                UniformCrossover{},
                GaussianMutation::new(mut_chance,mut_coeff).expect("Cannot instatiate mutation algorithm")
            )            
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
    
    fn process_brains(&mut self) {
        let foods = self.world.food.as_slice();
        // println!("Birds are thinking...");
        for (i, animal) in self.world.animals.iter_mut().enumerate() {
            // println!("- Bird {}:", i+1);
            let vision = animal.eye.process_vision(animal.position, animal.rotation, foods);
            // println!("\tVision: {:?}", vision);
            let brain_response = animal.brain.propagate(vision.as_slice());
            // println!("\tBrain Response: {:?}", brain_response);
            let speed = brain_response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
            let rotation = brain_response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);
            // println!("\tNew Speed: {:?}", speed);
            // println!("\tNew rotation: {:?}", rotation);
            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
            animal.rotation = Rotation2::new(animal.rotation.angle() + rotation);
        }
    }

    
    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_movements();
        self.process_collisions(rng);
        self.process_brains();
    }
    
    pub fn evolve(&mut self, rng: &mut dyn RngCore) {
        let population: Vec<AnimalIndividual> = self.world.animals().iter().map(|a| a.into()).collect();
        let new_population = self.evolution_algorithm.evolve(&population, rng).expect("Cannot evolve population");
        self.world.animals = new_population.iter().map(|i| Animal::from(i)).collect();
        for animal in &mut self.world.animals {
            animal.position = rng.gen();
            animal.rotation = rng.gen();
        }
    }
}
