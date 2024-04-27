use nalgebra::geometry::Point2;
use nalgebra::{Rotation2, wrap, distance};
use rand::{rngs, Rng, RngCore};


#[derive(Debug)]
pub struct Food {
   pub(crate) position: Point2<f32>
}


impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen()
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}
