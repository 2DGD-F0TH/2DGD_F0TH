class Tile{
    constructor(){
        this.x = null;
        this.y = null;
    }
}

function manhattan_distance(start, goal){
    return Math.abs(start.x - goal.x) + Math.abs(start.y - goal.y);
}
