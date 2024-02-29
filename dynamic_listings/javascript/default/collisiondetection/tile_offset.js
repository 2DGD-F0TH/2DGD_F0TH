class TiledPlayer{
    constructor(){
        this.offset = new Vector2D(0, 0);
        this.current_position = new Vector2D(10, 10);
        this.next_position = new Vector2D(10, 10);
    }

    update(dt){
        // ...
        // Check which direction is the player going
        if (KEYBOARD.Up_Arrow_Pressed){
            this.offset.y = -1;
        }
        if (KEYBOARD.Down_Arrow_Pressed){
            this.offset.y = 1;
        }
        if (KEYBOARD.Right_Arrow_Pressed){
            this.offset.x = 1;
        }
        if (KEYBOARD.Left_Arrow_Pressed){
            this.offset.x = -1;
        }
        // Get the destination tile
        this.next_position = this.current_position + this.offset;
        // Is the tile a wall?
        if (!MAP.get_tile(this.next_position).isWall()){
            // No, move the player to the new tile
            this.current_position = this.next_position;
        }
        // ...
    }
}
