# #![allow(dead_code)]
trait Handler {
    fn handle_request(&mut self) {
#       let condition = true;
        if condition {
            return self.real_handler();
        }

        if let Some(next) = self.next() {
            return next.handle_request();
        }
    }

    fn real_handler(&mut self);
    fn next(&mut self) -> &mut Option<Box<dyn Handler>>;
    fn add_handler(&mut self, new_handler: &dyn Handler);
}
