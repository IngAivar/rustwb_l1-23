struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(4.0, 6.0);

    let distance = p1.distance_to(&p2);
    println!("Расстояние между точками: {}", distance);
}