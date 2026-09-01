fn update(&mut self, dt: f32) {
    let vector_up = Vector2D::new(0., -1.);
    let vector_right = Vector2D::new(1., 0.);
    // ...
    let total_movement = vector_up + vector_right;
    self.character_controller.r#move(total_movement * dt);
    // ...
}
