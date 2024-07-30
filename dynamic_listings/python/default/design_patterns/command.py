class Command:
    # This is the abstract class that will be used as interface
    def execute(self) -> None:
        raise NotImplementedError("You should implement this!")


class JumpCommand(Command):
    # This will implement the execute method
    def execute(self) -> None:
        self.jump()

    def jump(self) -> None:
        # DO STUFF
        pass
