trait Shape {
    // A shape trait
    // An associated method that will be implemented by struct
    fn area(&self) -> f32;
    // An associated method that will be implemented by struct
    fn perimeter(&self) -> f32;
}

struct Rectangle {
    // A simple rectangle struct
    width: f32,
    height: f32,
}

impl Rectangle {
    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        // Returns the Area of the rectangle
        self.width * self.height
    }

    fn perimeter(&self) -> f32 {
        // Returns the Perimeter of the rectangle
        2. * (self.width + self.height)
    }
}

struct Circle {
    // A simple circle struct
    radius: f32,
}

impl Circle {
    fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        // Returns the Area of the circle
        3.1415 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        // Returns the circumference of the circle
        2. * 3.1415 * self.radius
    }
}
