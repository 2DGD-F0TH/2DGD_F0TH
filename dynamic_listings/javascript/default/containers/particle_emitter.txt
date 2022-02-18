class Emitter{
    /*
     * This is a simple particle emitter, it contains a list
     * of particles and it updates and manages them
     */

    constructor(location){
        this.origin = location;
        this.particles = new Array(8);  // We prepare 8 spaces for particles
    }

    update(dt){
        // Update the entire system, by updating each particle
        for (const particle of this.particles){
            if (!particle.is_dead()){
                particle.update(dt);
            }
        }
    }
}
