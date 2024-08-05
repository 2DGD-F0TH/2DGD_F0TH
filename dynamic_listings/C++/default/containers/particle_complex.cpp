class Particle{
    /*
     * This is a simple particle class, it contains a reference to
     * its texture, as well as some state
     */
    private:
        // The particle texture
        Texture texture;
        // Position, velocity and acceleration on the 2D plane
        Vector2D position;
        Vector2D velocity;
        Vector2D acceleration;
        // Lifespan of the particle
        float lifespan;
        // Current angle of rotation, and relative velocity and acceleration
        float rotation;
        float angular_velocity;
        float angular_acceleration;

        // Initial Status, for resetting
        Vector2D initial_velocity;
        float initial_rotation;
        float initial_a_vel;
        float initial_lifespan;

    public:
        Particle(Texture tex, Vector2D pos, Vector2D vel, Vector2D accel, float ls = 2000, float rot = 0, float a_vel = 0, float a_accel = 0){
            // We prepare the particle for usage
            texture = tex;
            position = pos;
            velocity = vel;
            acceleration = accel;
            lifespan = ls;  // About 2 seconds by default
            // We also prepare the reset variables: the position will be set by the emitter
            initial_lifespan = lifespan;
            initial_velocity = velocity;
            initial_a_vel = a_vel;
            initial_rotation = rotation;
        }

        void update(float dt){
            // We update the velocity (assuming dt is in milliseconds)
            velocity = velocity + acceleration;
            // Then the position
            position = position + velocity * dt;
            // Then the rotation
            angular_velocity = angular_velocity + angular_acceleration ;
            rotation = (rotation  + angular_velocity * dt) % 360;  // Wrap to zero when at 360 degrees
            // Now we update the lifespan of the particle;
            lifespan = lifespan - dt;
        }

        bool is_dead(){
            // Returns a boolean representing if the particle is dead
            return lifespan <= 0;
        }

        void reset(){
            // This function resets the initial status of the particle
            velocity = initial_velocity;
            rotation = initial_rotation;
            angular_velocity = initial_a_vel;
            lifespan = initial_lifespan;
        }
};
