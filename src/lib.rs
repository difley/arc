#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn line_point(origin: &Point, angle: &f32, distance: &f32) {
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn line_point_doesnt_explode() {
        let myPoint = Point { x: 2., y: 3. };
        assert_eq!(line_point(&myPoint, &0.4, &2.), ());
    }
}
