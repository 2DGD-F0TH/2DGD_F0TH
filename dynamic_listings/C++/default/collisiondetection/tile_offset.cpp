class TiledPlayer{
    private:
        Vector2D offset = Vector2D(0, 0);
        Vector2D current_position = Vector2D(10, 10);
        Vector2D next_position = Vector2D(10, 10);

    public:
        void update(float dt){
            // ...
            // Check which direction is the player going
            if (KEYBOARD.Up_Arrow_Pressed){
                offset.y = -1;
            }
            if (KEYBOARD.Down_Arrow_Pressed){
                offset.y = 1;
            }
            if (KEYBOARD.Right_Arrow_Pressed){
                offset.x = 1;
            }
            if (KEYBOARD.Left_Arrow_Pressed){
                offset.x = -1;
            }
            // Get the destination tile
            next_position = current_position + offset;
            // Is the tile a wall?
            if (!MAP.get_tile(next_position).isWall()){
                // No, move the player to the new tile
                current_position = next_position;
            }
            // ...
        }
};
