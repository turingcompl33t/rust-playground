// drop1.rs

#[derive(Debug)]
struct Point {
    x : i32,
    y : i32,
}

impl Point {
    fn new(x : i32, y : i32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        println!("Point at ({}, {}) dropped", self.x, self.y);
    }
}

fn main() {
    let _p1 = Point::new(1, 1);
    let _p2 = Point::new(2, 2);

    {
        let _p3 = Point::new(3, 3);
    }
}