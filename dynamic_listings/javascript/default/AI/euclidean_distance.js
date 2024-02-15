class Tile{
    constructor(){
        this.x = null;
        this.y = null;
    }
}

function euclidean_distance(start, goal){
    return Math.sqrt((start.x - goal.x) ** 2 + (start.y - goal.y) ** 2);
}
