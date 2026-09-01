fn update(&mut self, dt: f32) {
    let vector_up = Vector2D::new(0., -1.);
    let vector_right = Vector2D::new(1., 0.);
    // ...
    self.character_controller.r#move(vector_up * dt);
    self.character_controller.r#move(vector_right * dt);
    // ...
}
