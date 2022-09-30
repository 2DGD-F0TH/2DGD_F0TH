function solve_collision(player, object){
    /*
     * This algorithm solves a collision between the player
     * and an unmovable object
     * We are assuming the player is moving
     */
    // The overlap will help us decide how to react
    let overlap = get_overlap(player, object);
    if (overlap.x > overlap.y){
        // Y is the "shallow axis"
        if (player.speed.y > 0){
            // Player is going towards the bottom of screen
            player.rect.bottom = object.rect.top;
        }else{
            // Player is going towards the top of the screen
            player.rect.top = object.rect.bottom;
        }
    }else{
        // X is the "shallow axis"
        if (player.speed.x > 0){
            // Player is going right
            player.rect.right = object.rect.left;
        }else{
            // Player is going left
            player.rect.left = object.rect.left;
        }
    }
}
