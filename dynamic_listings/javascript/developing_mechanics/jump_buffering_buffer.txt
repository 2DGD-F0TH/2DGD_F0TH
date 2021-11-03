// ...
const jumpBufferTime = 5.0;
// ...
function update(dt){
    // ...
    if (controls.jump.isPressed()){
        player.hasBufferedJump = true;
        player.jumpBufferCountdown = jumpBufferTime;
    }
    // Take note on how this piece is outside the "jump is pressed" section
    if (player.hasBufferedJump){
        player.jumpBufferCountdown = player.jumpBufferCountdown - dt;
    }
    if (player.on_ground){
        if (player.jumpBufferCountdown > 0.0){
            // Jump
            player.jumpBufferCountdown = 0.0;
            player.hasBufferedJump = false;
        }
    }
    // ...
}
