/*
math-interpolate uses linear interpolation to interpolate two points to find the 
abscissa of the 3rd point.
*/

pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn interpolate(p1: Point, p2: Point, x: f32) -> f32 {
    (x - p2.x) / (p1.x - p2.x) * (p1.y - p2.y) + p2.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(5.0, 5.0);
        let x = 3.0;

        let result = interpolate(p1, p2, x);

        assert_eq!(result, 3.0);
    }
}
