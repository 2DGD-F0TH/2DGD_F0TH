local running = true

local Rectangle = {}
-- Rectangle class definition here ...

-- We create a display surface of 640x480 pixels
local screen_surface = engine.set_display({640, 480})

-- We keep the second rectangle a bit lower to be able to see both
local rectangle_1 = Rectangle:new({x=245,
                                   y=100,
                                   width=150,
                                   height=100,
                                   fill_color={0, 0, 255}})
local rectangle_2 = Rectangle:new({x=245,
                                   y=120,
                                   width=150,
                                   height=100,
                                   fill_color={0, 255, 0}})

-- This will be 1 for right and -1 for left
local movement_direction = 0
local rectangle_1_speed = 7
local rectangle_2_speed = 14


-- For ease, we assume we have an event queue we can process and we won't
-- take care of framerate limiting
while (running) do
    -- --------------- INPUT ---------------
    for event in event_queue do
        if (event.type == QUIT) then
            -- We are quitting the game
            running = false
        end
        if (event.type == KEYPRESS) then
            -- We are pressing a key
            if (event.key == RIGHT) then
                -- We are pressing the right key (moving the camera rightwards)
                movement_direction = -1
            end
            if (event.key == LEFT) then
                -- We are pressing the left key (moving the camera leftwards)
                movement_direction = 1
            end
        end
        if (event.type == KEYRELEASE) then
            if (event.key == RIGHT or event.key == LEFT) then
                movement_direction = 0
            end
        end
    end
    -- --------------- UPDATE ---------------
    if (movement_direction ~= 0) then
        rectangle_1.x = rectangle_1.x + (rectangle_1_speed * movement_direction)
        rectangle_2.x = rectangle_2.x + (rectangle_2_speed * movement_direction)
    end
    -- --------------- DRAW ---------------
    -- Fill the display with black
    screen_surface.fill({0, 0, 0})
    -- Draw the rectangles
    rectangle_1.draw_on(screen_surface)
    rectangle_2.draw_on(screen_surface)
    -- Show the result on screen
    screen_surface.display()
end
