#include <functional>
#include <cstdlib>

class Player{
    // ...
    public:
        void registerShootingObserver(std::function callback){
            // Function used to register an observer that will be called when the
            // player shoots a projectile.
        }
};

class JumpingBoss{
    // A boss that jumps when the player shoots. Sometimes.

    private:
        bool player_shot = false;
        bool on_ground = false;
        float y_velocity = 0.0;

    public:

        JumpingBoss(float x, float y, Player player){
            // ...
            player.registerShootingObserver(this.setPlayerShot);
            // ...
        }

        void setPlayerShot(){
            // Sets a state that tells the AI that the player shot a bullet
            player_shot = True;
        }

        void jump(){
            // Sets the boss velocity to -10, making it jump
            if (on_ground){
                y_velocity = -10;
            }
        }

        void update(float dt){
            // ...
            if (player_shot){
                if (std::rand() % 5 + 1 == 1){
                    // Jump only 20% of the times the player shoots
                    jump();
                }
            }
            // We reset player_shot to false, if we didn't the boss would jump
            // a lot more often than 20% of the time
            player_shot = False;
            // ...
        }
};
