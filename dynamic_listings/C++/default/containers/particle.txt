class Particle{
    /*
     * This is a simple particle class, it contains a reference to
     * its texture, as well as some state
     */
    private:
        Texture texture;
        Vector2D position;
        Vector2D velocity;
        Vector2D acceleration;
        float lifespan;

    public:
        Particle(Texture tex, Vector2D pos, Vector2D vel, Vector2D accel, float ls = 2000){
            // We prepare the particle for usage
            texture = tex;
            position = pos;
            velocity = vel;
            acceleration = accel;
            lifespan = ls;  // About 2 seconds by default
        }

        void update(float dt){
            // We update the velocity (assuming dt is in milliseconds)
            velocity = velocity + acceleration;
            // Then the position
            position = position + velocity * dt;
            // Now we update the lifespan of the particle;
            lifespan = lifespan - dt;
        }

        bool is_dead(){
            // Returns a boolean representing if the particle is dead
            return this.lifespan <= 0;
        }

        void setPosition(Vector2D pos){
            // Sets the particle position
            position = pos;
        }
};
