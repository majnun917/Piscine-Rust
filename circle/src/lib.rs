#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
   pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point(x, y),
            radius,
        }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

   pub fn intersect(&self, other: Circle) -> bool {
        let distance = self.center.distance(other.center);
        distance < (self.radius + other.radius) && distance > (self.radius - other.radius).abs()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(f64, f64);

impl Point {
    pub fn distance(self, other: Point) -> f64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// #[inline]
//     fn approx_eq(a: f64, b: f64) -> bool {
//         (a - b).abs() < f64::EPSILON
//     }

//     #[test]
//     fn test_new_circle() {
//         let circle = Circle::new(500.0, 400.0, 150.0);
//         assert!(approx_eq(circle.radius, 150.0));
//         assert!(approx_eq(circle.center.0, 500.0));
//         assert!(approx_eq(circle.center.1, 400.0));
//     }

//     #[test]
//     fn test_distance() {
//         let a = Point(0.0, 1.0);
//         let b = Point(0.0, 0.0);
//         assert!(approx_eq(a.distance(b), 1.0));

//         let a = Point(1.0, 0.0);
//         let b = Point(0.0, 0.0);
//         assert!(approx_eq(a.distance(b), 1.0));

//         let a = Point(1.0, 1.0);
//         let b = Point(0.0, 0.0);
//         assert!(approx_eq(a.distance(b), f64::sqrt(2.0)));
//     }

//     #[test]
//     fn test_area() {
//         let circle = Circle::new(500.0, 400.0, 150.0);
//         assert!(approx_eq(circle.area(), 70685.83470577035));
//     }

//     #[test]
//     fn test_intersection() {
//         let circle = Circle::new(500.0, 500.0, 150.0);
//         let circle1 = Circle::new(80.0, 115.0, 30.0);
//         assert!(!circle.intersect(circle1));

//         let circle = Circle::new(100.0, 300.0, 150.0);
//         let circle1 = Circle::new(80.0, 115.0, 100.0);
//         assert!(circle.intersect(circle1));
//     }
// }
