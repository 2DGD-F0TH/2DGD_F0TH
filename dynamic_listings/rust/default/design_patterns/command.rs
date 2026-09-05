# #![allow(dead_code)]
trait Command {
    fn execute(&self);
}

struct JumpCommand;

// This will implement the execute method
impl Command for JumpCommand {
    fn execute(&self) {
        self.jump();
    }
}

impl JumpCommand {
    fn jump(&self) {
        // DO STUFF
    }
}
