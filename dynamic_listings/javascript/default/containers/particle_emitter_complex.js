class Emitter{
    /*
     * This is a simple particle emitter, it contains a list
     * of particles and it updates and manages them
     */

    constructor(location, one_shot = false){
        this.origin = location;
        this.particles = new Array(8);  // We prepare 8 spaces for particles
        // Defines if this emitter streams continuously or only a burst of particles
        this.one_shot = one_shot;
    }

    update(dt){
        // Update the entire system, by updating each particle
        for (const particle of this.particles){
            if (this.one_shot){
                if (particle.is_dead()){
                    continue;
                }else{
                    particle.update(dt);
                }
            }else{
                if (particle.is_dead()){
                    particle.reset();  // Resets the state of the particle
                    particle.setPosition(this.origin);
                }
                particle.update(dt);
            }
        }
    }
}
