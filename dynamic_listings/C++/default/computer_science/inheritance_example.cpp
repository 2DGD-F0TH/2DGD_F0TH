class Shape{
    // An abstract shape class
    // An abstract function that will be overridden by subclasses
    virtual float area() = 0;
    // An abstract function that will be overridden by subclasses
    virtual float perimeter() = 0;
};

class Rectangle : public Shape{
    // A simple rectangle class
    float width;
    float height;

    Rectangle(float w, float h){
        width = w;
        height = h;
    }

    float area(){
        // Returns the Area of the rectangle
        return width * height;
    }

    float perimeter(){
        // Returns the Perimeter of the rectangle
        return 2 * (width + height);
    }
};

class Circle : public Shape{
    // A simple circle class
    float radius;

    Circle(float r){
        radius = r;
    }

    float area(){
        // Returns the Area of the circle
        return 3.1415 * 3.1415 * radius;
    }

    float perimeter(){
        // Returns the circumference of the circle
        return 2 * 3.1415 * radius;
    }
};
