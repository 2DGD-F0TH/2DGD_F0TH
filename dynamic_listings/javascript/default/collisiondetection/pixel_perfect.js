class Color{
    constructor(){
        this.colorData = [];
    }

    isWhite(){
        // ...
    };
}

class Bitmask{
    constructor(){
        this.data = [];  // An array of Color() classes
    }
    getColor(x, y){
        // ...
    };
    // ...
}

class Sprite{
    constructor(){
        this.bitmask = new Bitmask();
        this.x = null;
        this.y = null;
        this.width = 0;
        this.height = 0;
    }
}

function pixel_perfect_collision(A, B){
    // Calculate the intersecting rectangle to limit checks
    let x1 = max(A.x, B.x);
    let x2 = min((A.x + A.width), (B.x + B.width));

    let y1 = max(A.y, B.y);
    let y2 = min((A.y + A.height), (B.y + B.height));

    // For each pixel in the intersecting rectangle, let's check
    for (let y = y1; y <= y2; y++){
        for (let x = x1; x <= x2; x++){
            // We're working in the intersecting triangle, so we'll need to
            // rework our coordinates
            let a = A.bitmask.getColor(x - A.x, y - A.y);
            let b = B.bitmask.getColor(x - B.x, y - B.y);

            if (a.isWhite() && b.isWhite()){
                return true;
            }
        }
    }

    // If no collision is occurred by the end of the checking, we're safe
    return false;
}
