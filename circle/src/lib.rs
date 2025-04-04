#[derive(Debug, Clone, Copy)]

pub struct Circle {
	center :(f64,f64),
	radius :f64
}

impl Circle {
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0 
    }
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius *self.radius) as f64
    }
    pub fn intersect(&self, other: &Circle) -> bool {
        let dx = self.center.0 - other.center.0;
        let dy = self.center.1 - other.center.1;
        let distance = ((dx * dx + dy * dy) as f64).sqrt() as f64;
        let radius_sum = self.radius.abs() + other.radius.abs();
        distance <= radius_sum
    } 
    pub fn new(x: f64, y:f64,  radius: f64) -> Self {
        Circle{
            center:(x,y), 
            radius: radius
        }
    }
}

//for the points
pub struct Point{
    a:f64,
    b:f64
}   
impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let da = (self.a - other.a) as f64;
        let db = (self.b - other.b) as f64;
        (da * da + db * db).sqrt()
    }
    pub fn new(a: f64, b:f64) -> Self {
        Point{a, b}
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle::new(80.0, 115.0, 30.0);
        let point_a = Point::new(1.0, 1.0);
        let point_b = Point::new(0.0, 0.0);

        assert_eq!(circle.area(), 70685.83470577035);
        assert_eq!(circle.diameter(), 300.0);
        assert_eq!(circle1.diameter(), 60.0);
        assert_eq!(circle.intersect(&circle1), false);
        assert_eq!(point_a.distance(&point_b), 1.4142135623730951);
    }
}
