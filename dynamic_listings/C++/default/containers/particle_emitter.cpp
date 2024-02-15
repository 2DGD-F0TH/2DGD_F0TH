class Emitter{
    /*
     * This is a simple particle emitter, it contains a list
     * of particles and it updates and manages them
     */
    private:
        Vector2D origin;
        Particle[] particles;

    public:
        Emitter(Vector2D loc){
            origin = loc;
            particles = new Particle[8];  // We prepare 8 particles
        }

        function update(float dt){
            // Update the entire system, by updating each particle
            for (auto particle : particles) {
                if (!particle.is_dead()){
                    particle.update(dt);
                }
            }
        }
};
