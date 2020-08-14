bool lineLineCollision(float x1, float y1, float x2, float y2, float x3, float y3, float x4, float y4){
    // our previous implementation of the line/line collision detection
}

bool pointRectCollision(float x1, float y1, float rectx, float recty, float rectwidth, float rectheight){
    // our previous implementation of a point/rectangle collision detection
}

bool lineRectangleCollision(float x1, float y1, float x2, float y2, float rectx, float recty, float rectwidth, float rectheight){
    // If we want to test if a line is completely inside of a rect, we just need
    // to see if any of its endpoints is inside the rectangle
    if (pointRectCollision(x1, y1, rectx, recty, rectwidth, rectheight) || pointRectCollision(x2, y2, rectx, recty, rectwidth, rectheight)){
        // At least one of the ends of the segment is inside the rectangle
        return true;
    }

    // Now to test the rectangle against the line, if it's not completely inside
    bool left = lineLineCollision(x1, y1, x2, y2, rectx, recty, rectx, recty + rectheight);
    bool right = lineLineCollision(x1, y1, x2, y2, rectx + rectwidth, recty, rectx + rectwidth, recty + rectheight);
    bool top = lineLineCollision(x1, y1, x2, y2, rectx, recty, rectx + rectwidth, recty);
    bool bottom = lineLineCollision(x1, y1, x2, y2, rectx, recty + rectheight, rectx + rectwidth, recty + rectheight);

    if (left || right || top || bottom){
        // We hit one of the sides, we are colliding
        return true;
    }

    // In any other case, return false
    return false;
}
