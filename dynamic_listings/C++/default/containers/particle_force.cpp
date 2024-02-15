class Particle{
    /*
     * This is a simple particle class, now it has some mass
     * and a force application function
     */
    private:
        // ...
        Vector2D acceleration;
        // ...
        float mass;

    public:
        Particle(Texture texture, Vector2D position, Vector2D velocity, Vector2D acceleration, float lifespan = 2000, float rotation = 0, float a_vel = 0, float a_accel = 0, float m = 1){
            // We prepare the particle for usage the same way as earlier
            // ...
            mass = m;
        }

        // ...

        void applyForce(Vector2D force){
            // This function influences the acceleration by applying force
            Vector2D da = force / this.mass;
            acceleration = acceleration + da;
        }
};
