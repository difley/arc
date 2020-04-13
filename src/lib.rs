#[macro_use]
extern crate approx;

use approx::AbsDiffEq;
use approx::RelativeEq;
use approx::UlpsEq;
use std::f32;

#[derive(Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

impl AbsDiffEq for Point {

    type Epsilon = f32;

    fn default_epsilon() -> f32 {
        f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f32) -> bool {
        f32::abs_diff_eq(&self.x, &other.x, epsilon) &&
        f32::abs_diff_eq(&self.y, &other.y, epsilon)
    }
}

impl RelativeEq for Point {
    fn default_max_relative() -> f32 {
        1.
    }

    fn relative_eq(&self, other: &Self, epsilon: f32, max_relative: f32) -> bool {
        f32::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        f32::relative_eq(&self.y, &other.y, epsilon, max_relative)
    }
}

fn line_point(origin: &Point, angle: &f32, distance: &f32, t: &f32) -> Point {
    let x = origin.x + distance*angle.cos()*t;
    let y = origin.y + distance*angle.sin()*t;
    return Point {x, y};
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_point_doesnt_explode() {
        let my_point = Point { x: 2., y: 3. };
        abs_diff_eq!(line_point(&my_point, &0.4, &2., &1.), Point {x: 1., y: 2.}, epsilon = f32::EPSILON);
    }
}
