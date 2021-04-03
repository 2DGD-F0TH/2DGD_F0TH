class Command:
    # This is the abstract class that will be used as interface
    def execute(self):
        raise NotImplementedError("You should implement this!")


class JumpCommand(Command):
    # This will implement the execute method
    def execute(self):
        self.jump()

    def jump(self):
        # DO STUFF
        pass
