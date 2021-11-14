struct Point{
    // Rewritten as a memo
    int x;
    int y;
};

struct Rectangle{
    Point corner;
    int width;
    int height;
};

bool rect_rect_collision(Rectangle A, Rectangle B){
    if ((A.corner.x < B.corner.x + B.width) &&
            (A.corner.x + A.width > B.corner.x) &&
            (A.corner.y < B.corner.y + B.height) &&
            (A.corner.y + A.height > A.corner.y)){
        return true;
    }else{
        return false;
    }
}
