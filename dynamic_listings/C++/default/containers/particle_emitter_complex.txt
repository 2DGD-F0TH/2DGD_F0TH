class Emitter{
    /*
     * This is a simple particle emitter, it contains a list
     * of particles and it updates and manages them
     */
    private:
        Vector2D origin;
        Particle[] particles;
        // Defines if this emitter streams continuously or only a burst of particles
        bool one_shot = false;

    public:
        Emitter(Vector2D loc, boolean os = false){
            origin = loc;
            particles = new Particle[8];  // We prepare 8 particles
            one_shot = os;
        }

        void update(float dt){
            // Update the entire system, by updating each particle
            for (auto particle : particles) {
                if (one_shot){
                    if (particle.is_dead()){
                        continue;
                    }else{
                        particle.update(dt);
                    }
                }else{
                    if (particle.is_dead()){
                        particle.reset();  // Resets the state of the particle
                        particle.setPosition(origin);
                    }
                    particle.update(dt);
                }
            }
        }
};
