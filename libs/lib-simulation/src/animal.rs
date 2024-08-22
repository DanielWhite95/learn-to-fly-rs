use neural_network as nn;
use nalgebra::geometry::Point2;
use nalgebra::{Rotation2, wrap, distance};
use genetic_algorithm::{Individual, Chromosome};
use nn::{LayerTopology, NeuralNetwork};
use rand::{rngs, Rng, RngCore};

use crate::eye::{Eye, CELLS};

#[derive(Debug)]
pub struct Animal {
    pub(crate) eye: Eye,
    pub(crate) brain: nn::NeuralNetwork,
    pub(crate) position: Point2<f32>,
    pub(crate) rotation: Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) score: i32,
}


#[derive(Debug)]
pub struct Brain {
    nerual_network: NeuralNetwork
}

impl Brain {
    pub fn topology(eye: &Eye) -> Vec<LayerTopology> {
        vec![
            nn::LayerTopology{ neurons: eye.cells() * 2 }, // Input is vision for food and vision for other animals
            nn::LayerTopology{ neurons: 10 },
                nn::LayerTopology{ neurons: 2 } // Output is rotation angle and speed
            ]
    }
}

impl Animal {
    pub fn brain_topology_from_eyes(eye: &Eye) -> Vec<LayerTopology>{
        vec![
            nn::LayerTopology{ neurons: eye.cells() * 2 },
                        nn::LayerTopology{ neurons: 10 },
                nn::LayerTopology{ neurons: 2 } // Output is rotation angle and speed
            ]
    }


    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = nn::NeuralNetwork::random(
            Self::brain_topology_from_eyes(&eye),
            rng
        ).expect("Cannot build animal brain!");
        for layer in brain.get_layers() {
            for neuron in layer.get_neurons() {

            }
        }
        Self {
            eye,
            brain,
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            score: 0
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }
}

pub struct AnimalIndividual {
    fitness: f32,
    chromosome: Chromosome
}


impl Individual for AnimalIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn create(chromosome: Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome
        }
    }
}

impl From<&Animal> for AnimalIndividual {
    fn from(value: &Animal) -> Self {
        let animal_weigths = value.brain.weights().into_iter();
        Self {
            fitness: value.score as f32,
            chromosome: Chromosome::from_iter(animal_weigths)

        }
    }
}

impl From<&AnimalIndividual> for Animal {
    fn from(value: &AnimalIndividual) -> Self {
        let eye = Eye::default();
        let brain_topology = Self::brain_topology_from_eyes(&eye);
        let brain = NeuralNetwork::from_weights(&brain_topology,value.chromosome.genes()).expect("Cannot generate brain from weights");
        Self {
            score: value.fitness as i32,
            eye,
            brain,
            position: Point2::new(0.0, 0.0),
            rotation: Rotation2::new(0.0),
            speed: 0.001,

        }
    }
}
