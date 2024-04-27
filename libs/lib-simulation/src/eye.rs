use crate::*;
use std::f32::consts::*;
use nalgebra::{Rotation2,Vector2,Point2,wrap};

const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = FRAC_PI_4;
const CELLS: usize = 13;

pub struct Eye {
    fov_range: f32, 
    fov_angle: f32,
    cells: usize
}

impl Eye {
    
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range <= 1.0 && fov_range >= 0.0);
        assert!(cells > 0);
        Self { fov_range, fov_angle, cells }
    }
    
    pub(crate) fn process_vision(&self, position: Point2<f32>, rotation: Rotation2<f32>, foods: &[Food]) -> Vec<f32> {
        let mut activated_cells: Vec<f32> = (0..self.cells).map(|_| 0.0).collect();
        
        for f in foods.iter() {
            let dist_vec = f.position - position;
            let dist = dist_vec.norm();
            if dist > self.fov_range {
                return activated_cells;
            }
            let angle = Rotation2::rotation_between(&Vector2::y(), &dist_vec).angle() - rotation.angle();
            let angle = wrap(angle, -PI, PI);
            if angle < - self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                return activated_cells;
            }
            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(self.cells - 1);
            let energy = (self.fov_range - dist) / self.fov_range;
            activated_cells[cell] += energy;
        }
        activated_cells
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

}


impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn vision_to_string_repr(vision: &[f32]) -> String {
            let chars: Vec<_> = vision.iter().map(|&f| if f > 0.7 {
                "*"
            } else if f > 0.3 {
                "+"
            } else if f > 0.0 {
                "" 
            } else {
                " "
            }).collect();
            println!("{:?}", chars);
            chars.join("")
        }
        
        fn run(self) {
            let eye = Eye::default();
            let actual_vision = eye.process_vision(Point2::new(self.x, self.y), Rotation2::new(self.rot), self.foods.as_slice());
            let actual_vision = Self::vision_to_string_repr(&actual_vision);
            assert_eq!(actual_vision.as_str(), self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            position: Point2::new(x, y),
        }
    }

    #[test_case(1.0, "      +      ")] // Food is inside the FOV
    #[test_case(0.9, "      +      ")] // ditto
    #[test_case(0.8, "      +      ")] // ditto
    #[test_case(0.7, "      .      ")] // Food slowly disappears
    #[test_case(0.6, "      .      ")] // ditto
    #[test_case(0.5, "             ")] // Food disappeared!
    #[test_case(0.4, "             ")]
    #[test_case(0.3, "             ")]
    #[test_case(0.2, "             ")]
    #[test_case(0.1, "             ")]
    fn fov_ranges(fov_range: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.5, 1.0)],
            fov_range,
            fov_angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rot: 0.0, 
            expected_vision: expected_vision 
        }.run()
    }
}
