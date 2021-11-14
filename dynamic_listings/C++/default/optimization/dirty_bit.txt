class Player{
    private:
        Vector speed(0, 0);
        bool needs_update = false;
    // ...
    public:
        void input(){
            // ...
            if (Keyboard.get(RIGHT_KEY).is_Pressed){
                speed = speed + Vector(1, 0);  // Move right
                needs_update = true;
            }
            // ...
            if (Keyboard.get(UP_KEY).is_Pressed){
                speed = speed + Vector(0, -100);  // Move up (jump)
                needs_update = true;
            }
            // ...
        }

        void update(float dt){
            if (needs_update){
                // Do Update instructions
                // ...
                //
            }
        }
};
