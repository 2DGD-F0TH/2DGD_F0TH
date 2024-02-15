function point_triangle_collision(px, py, x1, y1, x2, y2, x3, y3){
    let original_area = Math.abs((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1));
    let area1 = Math.abs((x1-px)*(y2-py) - (x2-px)*(y1-py));
    let area2 = Math.abs((x2-px)*(y3-py) - (x3-px)*(y2-py));
    let area3 = Math.abs((x3-px)*(y1-py) - (x1-px)*(y3-py));
    if (area1 + area2 + area3 == original_area){
        return true;
    }else{
        return false;
    }
}
