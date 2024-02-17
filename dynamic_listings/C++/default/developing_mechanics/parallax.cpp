bool running = true;

class RGB{
    // ...
};

class Rectangle{
    // ...
};

// We create a display surface of 640x480 pixels
Surface screen_surface = engine.set_display((640, 480));

// We keep the second rectangle a bit lower to be able to see both
Rectangle rectangle_1 = Rectangle(245,  // x
                                  100,  // y
                                  150,  // width
                                  100,  // height
                                  RGB(0, 0, 255)  // fill color
                                  );
Rectangle rectangle_2 = Rectangle(245,  // x
                                  120,  // y
                                  150,  // width
                                  100,  // height
                                  RGB(0, 255, 0) // fill color
                                  );
float rectangle_1_speed = 7;
float rectangle_2_speed = 14;

// This will be 1 for right and -1 for left
int movement_direction = 0;


// For ease, we assume we have an event queue we can process and we won't
// take care of framerate limiting
while (running){
    // --------------- INPUT ---------------
    for event in event_queue{
        if (event.type == QUIT){
            // We are quitting the game
            running = false;
        }
        if (event.type == KEYPRESS){
            // We are pressing a key
            if (event.key == RIGHT){
                // We are pressing the right key (moving the camera rightwards)
                movement_direction = -1;
            }
            if (event.key == LEFT){
                // We are pressing the left key (moving the camera leftwards)
                movement_direction = 1;
            }
        }
        if (event.type == KEYRELEASE){
            if (event.key == RIGHT or event.key == LEFT){
                movement_direction = 0;
            }
        }
    }
    // --------------- UPDATE ---------------
    if (movement_direction != 0){
        rectangle_1.x += rectangle_1_speed * movement_direction;
        rectangle_2.x += rectangle_2_speed * movement_direction;
    }
    // --------------- DRAW ---------------
    // Fill the display with black
    screen_surface.fill(RGB(0, 0, 0))
    // Draw the rectangles
    rectangle_1.draw_on(screen_surface);
    rectangle_2.draw_on(screen_surface);
    // Show the result on screen
    screen_surface.display();
}
