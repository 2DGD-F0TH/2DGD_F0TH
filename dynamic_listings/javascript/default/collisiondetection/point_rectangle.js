function pointRectCollision(x1, y1, rectx, recty, rectwidth, rectheight){
    // We check if the point is inside the rectangle
    return x1 >= rectx && x1 <= rectx + rectwidth && y1 >= recty && y1 <= recty + rectheight;
}
