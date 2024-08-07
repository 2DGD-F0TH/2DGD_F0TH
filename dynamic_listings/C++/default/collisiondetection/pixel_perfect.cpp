#include<algorithm>
struct Color{
    int colorData;
    bool isWhite{
        //...
    }
};

struct Bitmask{
    Color data[];
    Color getColor(int x, int y){
        // ...
    }
};

struct Sprite{
    Bitmask bitmask;
    int x;
    int y;
    int width;
    int height;
};


bool pixel_perfect_collision(Sprite A, Sprite B){
    // Calculate the intersecting rectangle to limit checks
    int x1 = std::max(A.x, B.x);
    int x2 = std::min((A.x + A.width), (B.x + B.width));

    int y1 = std::max(A.y, B.y);
    int y2 = std::min((A.y + A.height), (B.y + B.height));

    // For each pixel in the intersecting rectangle, let's check
    for (int y=y1; y < y2; y++){
        for (int x=x1; x < x2; x++){
            // We're working in the intersecting triangle, so we'll need to
            // rework our coordinates
            Color a = A.bitmask.getColor(x - A.x, y - A.y);
            Color b = B.bitmask.getColor(x - B.x, y - B.y);
            if (a.isWhite() && b.isWhite()){
                return true;
            }
        }
    }

    // If no collision is occurred by the end of the checking, we're safe
    return false;
}
