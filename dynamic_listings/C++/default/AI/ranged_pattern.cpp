#include <random>

class Boss{
    private:
        Player player;
        bool player_too_close;
        float base_movement_velocity;
        float too_close_space;
        Vector2D velocity;
        Vector2D position;

    public:
        Boss(Player player){
            player = player;
            player_too_close = false;
            base_movement_velocity = 10;
            too_close_space = 20;
            velocity = Vector2D();
            position = Vector2D();
        }

        void update(float dt){
            // ...
            if (abs(player.position.x - position.x) < too_close_space){
                if (abs(player.position.y - position.y) < too_close_space){
                    // The player is too close
                    if (std::rand() % 5 + 1 == 1){
                        // Add a bit of randomization
                        player_too_close = true;
                    }
                }
            }
            // We're using a variable to preserve the "too close" state between frames
            if (player_too_close){
                // The player is too close, make some distance
                Vector2D distance = position - player.position;
                // Make it a direction
                Vector2D direction = distance.normalize();
                // is the direction the boss should go now, transfer it to velocity
                velocity = direction * base_movement_velocity;
            }
            // ...
            // The boss and player now have moved, let's see if they're far enough
            // ...
            if (abs(player.position.x - position.x) > too_close_space){
                if (abs(player.position.y - position.y) > too_close_space){
                    // The player is far enough now
                    player_too_close = false;
                }
            }
        }
};
