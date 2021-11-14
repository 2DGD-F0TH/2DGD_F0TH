bool pointRectCollision(float x1, float y1, float rectx, float recty, float rectwidth, float rectheight){
    // We check if the point is inside the rectangle
    return x1 >= rectx && x1 <= rectx + rectwidth && y1 >= recty && y1 <= recty + rectheight;
}
