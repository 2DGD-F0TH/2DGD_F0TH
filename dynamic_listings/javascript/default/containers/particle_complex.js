class Particle{
    /*
     * This is a simple particle class, it contains a reference to
     * its texture, as well as some state
     */

    constructor(texture, position, velocity, acceleration, lifespan = 2000, rotation = 0, a_vel = 0, a_accel = 0){
        // We prepare the particle for usage
        // The particle texture
        this.texture = texture;
        // Position, velocity and acceleration on the 2D plane
        this.position = position;
        this.velocity = velocity;
        this.acceleration = acceleration;
        // Lifespan of the particle
        this.lifespan = lifespan;  // About 2 seconds by default
        // Current angle of rotation, and relative velocity and acceleration
        this.rotation = rotation;
        this.angular_velocity = a_vel;
        this.angular_acceleration = a_accel;
        // We also prepare the reset variables: the position will be set by the emitter
        this.initial_lifespan = lifespan;
        this.initial_velocity = velocity;
        this.initial_a_vel = a_vel;
        this.initial_rotation = rotation;
    }

    update(dt){
        // We update the velocity (assuming dt is in milliseconds)
        this.velocity = this.velocity + this.acceleration;
        // Then the position
        this.position = this.position + this.velocity * dt;
        // Then the rotation
        this.angular_velocity = this.angular_velocity + this.angular_acceleration ;
        this.rotation = (this.rotation  + this.angular_velocity * dt) % 360;  // Wrap to zero when at 360 degrees
        // Now we update the lifespan of the particle;
        this.lifespan = this.lifespan - dt;
    }

    is_dead(){
        // Returns a boolean representing if the particle is dead
        return this.lifespan <= 0;
    }

    reset(){
        // This function resets the initial status of the particle
        this.velocity = this.initial_velocity;
        this.rotation = this.initial_rotation;
        this.a_vel = this.initial_a_vel;
        this.lifespan = this.initial_lifespan;
    }
}
