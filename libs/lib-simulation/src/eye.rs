use crate::*;
use std::f32::consts::*;
use nalgebra::{Rotation2,Vector2,Point2,wrap};

pub(crate) const FOV_RANGE: f32 = 0.75;
pub(crate) const FOV_ANGLE: f32 = FRAC_PI_4;
pub(crate) const CELLS: usize = 13;

#[derive(Debug)]
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

    pub(crate) fn process_vision(&self, position: Point2<f32>, rotation: Rotation2<f32>, elements: &[Point2<f32>]) -> Vec<f32> {
        let mut activated_cells: Vec<f32> = (0..self.cells).map(|_| 0.0).collect();

        for f in elements.iter().filter(|f| !position.eq(f)) {
            let dist_vec = f - position;
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
            let chars: Vec<_> = vision.iter().map(|&f| if f >= 0.7 {
                "#"
            } else if f >= 0.3 {
                "+"
            } else if f > 0.0 {
                "."
            } else {
                " "
            }).collect();
            println!("{:?}", chars);
            chars.join("")
        }

        fn run(self) {
            let eye = Eye {fov_range: self.fov_range, fov_angle: self.fov_angle, ..Default::default()};
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

    #[test_case(0.00 * PI, "         +   ")] // Food is to our right
    #[test_case(0.25 * PI, "        +    ")]
    #[test_case(0.50 * PI, "      +      ")] // Food is in front of us
    #[test_case(0.75 * PI, "    +        ")]
    #[test_case(1.00 * PI, "   +         ")] // Food is to our left
    #[test_case(1.25 * PI, " +           ")]
    #[test_case(1.50 * PI, "            +")] // Food is behind us
    #[test_case(1.75 * PI, "           + ")] // (we continue to see it
    #[test_case(2.00 * PI, "         +   ")] // due to 360° fov_angle.)
    #[test_case(2.25 * PI, "        +    ")]
    #[test_case(2.50 * PI, "      +      ")]
    fn rotations(rot: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(0.0, 0.5)],
            fov_range: 1.0,
            fov_angle: 2.0 * PI,
            x: 0.5,
            y: 0.5,
            rot,
            expected_vision,
        }.run()
    }

    // Checking the X axis:
    // (you can see the bird is "flying away" from the foods)
    #[test_case(0.9, 0.5, "#           #")]
    #[test_case(0.8, 0.5, "  #       #  ")]
    #[test_case(0.7, 0.5, "   +     +   ")]
    #[test_case(0.6, 0.5, "    +   +    ")]
    #[test_case(0.5, 0.5, "    +   +    ")]
    #[test_case(0.4, 0.5, "     + +     ")]
    #[test_case(0.3, 0.5, "     . .     ")]
    #[test_case(0.2, 0.5, "     . .     ")]
    #[test_case(0.1, 0.5, "     . .     ")]
    #[test_case(0.0, 0.5, "             ")]
    //
    // Checking the Y axis:
    // (you can see the bird is "flying alongside" the foods)
    #[test_case(0.5, 0.0, "            +")]
    #[test_case(0.5, 0.1, "          + .")]
    #[test_case(0.5, 0.2, "         +  +")]
    #[test_case(0.5, 0.3, "        + +  ")]
    #[test_case(0.5, 0.4, "      +  +   ")]
    #[test_case(0.5, 0.6, "   +  +      ")]
    #[test_case(0.5, 0.7, "  + +        ")]
    #[test_case(0.5, 0.8, "+  +         ")]
    #[test_case(0.5, 0.9, ". +          ")]
    #[test_case(0.5, 1.0, "             ")]
    fn positions(x: f32, y: f32, expected_vision: &'static str) {
        TestCase {
            foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
            fov_range: 1.0,
            fov_angle: FRAC_PI_2,
            rot: 3.0 * FRAC_PI_2,
            x,
            y,
            expected_vision,
        }.run()
    }
}
