#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    /// The function returns a new circle.
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        let new  = Circle{center: Point(x, y), radius: radius};
        return new;
    }

    /// returns the diameter of the circle.
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    /// returns the area of the circle
    pub fn area(&self) -> f64 {
        const PI: f64 = 3.141592653589793;
        PI * self.radius.powi(2)
    }

    /// returns if two circles intersect
    pub fn intersect(&self, crl: Self) -> bool {
        let distance = self.center.distance(crl.center);
        if distance > (self.radius + crl.radius) || distance < (self.radius - crl.radius) {
            return false
        } else { 
            return true
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    /// returns the distance between two coordinates.
    pub fn distance(&self, to: Self) -> f64  {
        ((to.0 - self.0).powi(2) + (to.1 - self.1).powi(2)).sqrt()
    }
}
