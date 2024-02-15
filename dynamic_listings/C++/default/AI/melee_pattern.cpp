#include <random>

class Boss{
    private:
        Player player;
        bool player_too_far;
        float base_movement_velocity;
        float too_far_space;
        Vector2D velocity;
        Vector2D position;

    public:
        Boss(Player player){
            player = player;
            player_too_far = false;
            base_movement_velocity = 10;
            too_far_space = 30;
            velocity = Vector2D();
            position = Vector2D();
        }

        void update(float dt){
            // ...
            if (abs(player.position.x - position.x) > too_far_space){
                if (abs(player.position.y - position.y) > too_far_space){
                    // The player is too close
                    if (std::rand() % 5 + 1 == 1){
                        // Add a bit of randomization
                        player_too_far = true;
                    }
                }
            }
            // We're using a variable to preserve the "too far" state between frames
            if (player_too_far){
                // The player is too far, close in
                Vector2D distance = player.position - position;
                // Make it a direction
                Vector2D direction = distance.normalize();
                // is the direction the boss should go now, transfer it to velocity
                velocity = direction * base_movement_velocity;
            }
            // ...
            // The boss and player now have moved, let's see if they're close enough
            // ...
            if (abs(player.position.x - position.x) < too_far_space){
                if (abs(player.position.y - position.y) < too_far_space){
                    // The player is close enough now
                    player_too_far = false;
                }
            }
        }
};
