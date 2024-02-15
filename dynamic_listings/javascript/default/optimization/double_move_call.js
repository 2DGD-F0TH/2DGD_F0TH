function update(dt){
    let vector_up = new Vector(0, -1);
    let vector_right = new Vector(1, 0);
    // ...
    characterController.Move(vector_up * dt);
    characterController.Move(vector_right * dt);
    // ...
}
