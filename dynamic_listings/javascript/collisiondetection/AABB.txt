class Point{
    // Rewritten as a memo
    constructor(){
        this.x = null;
        this.y = null;
    }
}

class Rectangle{
    constructor(){
        this.corner = new Point();
        this.width = 0;
        this.height = 0;
    }
}

function rect_rect_collision(A, B){
    if ((A.corner.x < B.corner.x + B.width) &&
       (A.corner.x + A.width > B.corner.x) &&
       (A.corner.y < B.corner.y + B.height) &&
       (A.corner.y + A.height > A.corner.y)){
        return True;
    }else{
        return False;
    }
}
