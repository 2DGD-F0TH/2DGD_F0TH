class Particle{
    /*
     * This is a simple particle class, it contains a reference to
     * its texture, as well as some state
     */
    constructor(texture, position, velocity, acceleration, lifespan = 2000){
        // We prepare the particle for usage
        this.texture = texture;
        this.position = position;
        this.velocity = velocity;
        this.acceleration = acceleration;
        this.lifespan = lifespan;  // About 2 seconds by default
    }

    update(dt){
        // We update the velocity (assuming dt is in milliseconds)
        this.velocity = this.velocity + this.acceleration;
        // Then the position
        this.position = this.position + this.velocity * dt;
        // Now we update the lifespan of the particle;
        this.lifespan = this.lifespan - dt;
    }

    is_dead(){
        // Returns a boolean representing if the particle is dead
        return this.lifespan <= 0;
    }

    setPosition(position){
        // Sets the particle position
        this.position = position;
    }
}
