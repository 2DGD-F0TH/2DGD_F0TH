# struct Screen;
# impl Screen {
#     fn initialize(&mut self, _: u32, _: u32, _: u32, _: bool) {}
# }
# fn main() {
# let mut screen = Screen;
// ...
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;
const BITS_PER_PIXEL: u32 = 32;
const FULLSCREEN: bool = false;
screen.initialize(SCREEN_WIDTH, SCREEN_HEIGHT, BITS_PER_PIXEL, FULLSCREEN);
// ...
# }
