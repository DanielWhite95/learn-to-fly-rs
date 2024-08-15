use lib_simulation;
use wasm_bindgen::prelude::*;
use rand::prelude::*;

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
    "Mister Peanutbutter".into()
}

#[wasm_bindgen]
struct Simulation {
    rng: ThreadRng,
    sim: lib_simulation::Simulation
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(animals: usize, food: usize, mut_chance: f32, mut_coeff: f32) -> Self {
        let mut rng = thread_rng();
        let sim = lib_simulation::Simulation::random(&mut rng, animals, food, mut_chance, mut_coeff, 50);

        Self { rng, sim }
    }

    pub fn world(&self) -> World {
        World::from(self.sim.world())
    }


    pub fn step(&mut self) {
        self.sim.step(&mut self.rng);
    }
}

#[wasm_bindgen]
#[derive(Clone,  Debug)]
pub struct Food {
    pub x: f32,
    pub y: f32
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Animal {
    pub x: f32,
    pub y: f32
}

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
    #[wasm_bindgen(getter_with_clone)]
    pub animals: Vec<Animal>,
    #[wasm_bindgen(getter_with_clone)]
    pub food: Vec<Food>
}

impl From<&lib_simulation::Food> for Food {
    fn from(value: &lib_simulation::Food) -> Self {
        let orig_pos = value.position();
        Self {
            x: orig_pos.x,
            y: orig_pos.y
        }
    }
}

impl From<&lib_simulation::Animal> for Animal {
    fn from(value: &lib_simulation::Animal) -> Self {
        let orig_pos = value.position();
        Self {
            x: orig_pos.x,
            y: orig_pos.y
        }
    }
}


impl From<&lib_simulation::World> for World {
    fn from(orig: &lib_simulation::World) -> Self {
        Self {
            animals: orig.animals().iter().map(Animal::from).collect(),
            food: orig.food().iter().map(Food::from).collect()
        }
    }
}
