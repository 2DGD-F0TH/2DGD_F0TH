class Shape{
    // An abstract shape class
    area(){
        // An abstract function that will be overridden by subclasses
        throw "Not Implemented";
    }
    perimeter(){
        // An abstract function that will be overridden by subclasses
        throw "Not Implemented";
    }
};

class Rectangle extends Shape{
    // A simple rectangle class

    constructor(w, h){
        this.width = w;
        this.height = h;
    }

    area() {
        // Returns the Area of the rectangle
        return this.width * this.height;
    }

    perimeter() {
        // Returns the Perimeter of the rectangle
        return 2 * (this.width + this.height);
    }
}

class Circle extends Shape{
    // A simple circle class

    constructor(r){
        this.radius = r;
    }

    area() {
        // Returns the Area of the circle
        return this.radius * Math.PI ** 2;
    }

    perimeter() {
        // Returns the circumference of the circle
        return 2 * Math.PI * this.radius;
    }
}
