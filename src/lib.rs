#[macro_use]
extern crate approx;

use approx::AbsDiffEq;
use approx::RelativeEq;
use std::f64;

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl AbsDiffEq for Point {

    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        f64::abs_diff_eq(&self.x, &other.x, epsilon) &&
        f64::abs_diff_eq(&self.y, &other.y, epsilon)
    }
}

impl RelativeEq for Point {
    fn default_max_relative() -> f64 {
        1.
    }

    fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
        f64::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        f64::relative_eq(&self.y, &other.y, epsilon, max_relative)
    }
}

fn line_point(origin: &Point, angle: &f64, distance: &f64, t: &f64) -> Point {
    let x = origin.x + distance*angle.cos()*t;
    let y = origin.y + distance*angle.sin()*t;
    return Point {x, y};
}

fn arc_point(center: &Point, radius: &f64, angle_start: &f64, angle_stop: &f64, t: &f64) -> Point {
    let angle = (angle_stop - angle_start) * t + angle_start;
    return Point {
        x: center.x + radius*angle.cos(),
        y: center.y + radius*angle.sin()
    };
}
 
#[cfg(test)]
mod tests {
    use super::*;
    use f64::consts;

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
        assert_abs_diff_eq!(line_point(&my_point, &(consts::PI/4.), &1., &1.), Point { x: 1./f64::sqrt(2.), y: 1./f64::sqrt(2.) });
    }

    #[test]
    fn translated_vertical_line() {
        let my_point = Point { x: 5.5, y: 1.1 };
        assert_abs_diff_eq!(line_point(&my_point, &(consts::PI/2.), &1., &1.), Point { x: 5.5, y: 2.1 });
    }

    #[test]
    fn halfway_line() {
        let my_point = Point { x: 0., y: 0. };
        assert_abs_diff_eq!(line_point(&my_point, &0., &1., &0.5), Point {x: 0.5, y: 0.});
    }

    #[test]
    fn half_circle_at_origin() {
        let arc_terminal_point = arc_point(&Point{ x: 0., y: 0. }, &1., &0., &(consts::PI), &1.);
        assert_abs_diff_eq!(arc_terminal_point, Point { x: -1., y: 0. });
    }

    #[test]
    fn half_circle_at_1_1() {
        let arc_terminal_point = arc_point(&Point{ x: 1., y: 1. }, &1., &0., &(consts::PI), &1.);
        assert_abs_diff_eq!(arc_terminal_point, Point { x: 0., y: 1. });
    }

    #[test]
    fn half_circle_at_origin_halfway_point() {
        let arc_halfway_point = arc_point(&Point{ x: 0., y: 0. }, &1., &0., &(consts::PI), &0.5);
        assert_abs_diff_eq!(arc_halfway_point, Point { x: 0., y: 1. });
    }

    #[test]
    fn half_circle_at_origin_radius_hal_half() {
        let arc_terminal_point = arc_point(&Point{ x: 0., y: 0. }, &0.5, &0., &(consts::PI), &1.);
       assert_abs_diff_eq!(arc_terminal_point, Point { x: -0.5, y: 0. });
    }
}
