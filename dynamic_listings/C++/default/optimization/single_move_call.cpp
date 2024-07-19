void update(float dt){
    Vector vector_up(0, -1);
    Vector vector_right(1, 0);
    // ...
    Vector total_movement = vector_up + vector_right;
    characterController.move(total_movement * dt);
    // ...
}
