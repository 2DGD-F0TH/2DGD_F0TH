# #![allow(dead_code)]
struct Rgb {
    // ...
}
# impl Rgb {
#     fn new(_: u8, _: u8, _: u8) -> Self { todo!() }
# }

struct Rectangle{
    // ...
#   x: f32,
#   y: f32,
}
# impl Rectangle {
#    fn new(_: u32, _: u32, _: u32, _: u32, _: Rgb) -> Self { todo!() }
#    fn draw_on(&self, _: &mut Surface) {}
# }

# #[derive(PartialEq)]
# enum EventType {
#     Quit,
#     Keypress,
#     Keyrelease,
# }
# #[derive(PartialEq)]
# enum Key {
#     Right,
#     Left,
# }
# struct Event {
#     ty: EventType,
#     key: Key,
# }
# struct Surface;
# impl Surface {
#     fn fill(&mut self, _: Rgb) {}
#     fn display(&self) {}
# }
# #[derive(Default)]
# struct Engine {
#     event_queue: Vec<Event>,
# }
# impl Engine {
#     fn set_display(&self, _: (u32, u32)) -> Surface { todo!() }
# }
# fn main() {
# let engine = Engine::default();
# let event_queue = &engine.event_queue;
let mut running = true;

// We create a display surface of 640x480 pixels
let mut screen_surface = engine.set_display((640, 480));

// We keep the second rectangle a bit lower to be able to see both
let mut rectangle_1 = Rectangle::new(
    245,  // x
    100,  // y
    150,  // width
    100,  // height
    Rgb::new(0, 0, 255)  // fill color
);
let mut rectangle_2 = Rectangle::new(
    245,  // x
    120,  // y
    150,  // width
    100,  // height
    Rgb::new(0, 255, 0) // fill color
);
let rectangle_1_speed = 7.;
let rectangle_2_speed = 14.;

// This will be 1 for right and -1 for left
let mut movement_direction = 0;

// For ease, we assume we have an event queue we can process and we won't
// take care of framerate limiting
while running {
    // --------------- INPUT ---------------
    for event in event_queue {
        if event.ty == EventType::Quit {
            // We are quitting the game
            running = false;
        }
        if event.ty == EventType::Keypress {
            // We are pressing a key
            if event.key == Key::Right {
                // We are pressing the right key (moving the camera rightwards)
                movement_direction = -1;
            }
            if event.key == Key::Left {
                // We are pressing the left key (moving the camera leftwards)
                movement_direction = 1;
            }
        }
        if event.ty == EventType::Keyrelease {
            if event.key == Key::Right || event.key == Key::Left {
                movement_direction = 0;
            }
        }
    }
    // --------------- UPDATE ---------------
    if movement_direction != 0 {
        rectangle_1.x += rectangle_1_speed * movement_direction as f32;
        rectangle_2.x += rectangle_2_speed * movement_direction as f32;
    }
    // --------------- DRAW ---------------
    // Fill the display with black
    screen_surface.fill(Rgb::new(0, 0, 0));
    // Draw the rectangles
    rectangle_1.draw_on(&mut screen_surface);
    rectangle_2.draw_on(&mut screen_surface);
    // Show the result on screen
    screen_surface.display();
}
# }
