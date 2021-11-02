function update(dt){
    let vector_up = new Vector(0, -1);
    let vector_right = new Vector(1, 0);
    // ...
    let total_movement = vector_up + vector_right;
    characterController.Move(total_movement * dt);
    // ...
}
