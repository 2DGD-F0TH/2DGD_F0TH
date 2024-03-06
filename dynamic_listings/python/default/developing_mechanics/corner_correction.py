def update(dt):
    # Using the brute force checking for simplicity
    for block in player.findCollisions(blocks):
        # ...
        if player.is_jumping:
            # We are jumping, we need to check if we are ascending
            # this way we will avoid "bonking our head" on a pixel
            if player.velocity.y < 0:
                # We know we are ascending, let's check how far we are
                # from the borders and react accordingly
                if player.position.x > block.rect.right - 5:
                    # The player's left side is penetrating the block by
                    # less than 5 pixels, let's correct it
                    player.position.x = block.rect.right
                elif player.position.x + player.width < block.rect.left + 5:
                    # The player's right side is penetrating the block by
                    # less than 5 pixels, let's correct it
                    player.position.x = block.rect.left - player.width
                else:
                    # The player is totally colliding with (bonking its head on) the block,
                    # without need for corner correction, let's just act normally
                    player.velocity.y = 0
                    player.position.y = block.rect.bottom
    # ...
