class Particle{
    /*
     * This is a simple particle class, now it has some mass
     * and a force application function
     */
    // ...

    constructor(texture, position, velocity, acceleration, lifespan = 2000, rotation = 0, a_vel = 0, a_accel = 0, mass = 1){
        // We prepare the particle for usage the same way as earlier
        // ...
        this.mass = mass;
    }

    // ...

    applyForce(force){
        // This function influences the acceleration by applying force
        let da = force / this.mass;
        this.acceleration = this.acceleration + da;
    }
}
