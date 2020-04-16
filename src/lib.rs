#[macro_use]
extern crate approx;

use approx::AbsDiffEq;
use approx::RelativeEq;
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
    use f32::consts;

    #[test]
    fn vertical_line_at_origin() {
        let my_point = Point { x: 0., y: 0. };
        assert_abs_diff_eq!(line_point(&my_point, &(consts::PI/2.), &1., &1.), Point {x: 0., y: 1.});
    }

    #[test]
    fn horizontal_line_at_origin() {
        let my_point = Point { x: 0., y: 0. };
        assert_abs_diff_eq!(line_point(&my_point, &consts::PI, &1., &1.), Point { x: -1., y: 0. });
    }

    #[test]
    fn halfway_between_vertical_and_horizontal() {
        let my_point = Point { x: 0., y: 0. };
        assert_abs_diff_eq!(line_point(&my_point, &(consts::PI/4.), &1., &1.), Point { x: 1./f32::sqrt(2.), y: 1./f32::sqrt(2.) });
    }

    #[test]
    fn translated_vertical_line() {
        let my_point = Point { x: 5.5, y: 1.1 };
        assert_abs_diff_eq!(line_point(&my_point, &(consts::PI/2.), &1., &1.), Point { x: 5.5, y: 2.1 });
    }
}
