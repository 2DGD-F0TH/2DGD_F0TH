from typing import Self


class Handler:
    # This is the handler abstract/class interface that the sender connects to
    next_handler: Self = None  # The next handler in the chain

    def handle_request(self) -> None:
        if condition:
            # In case I can handle this request
            return self.real_handler()

        if self.next_handler:
            return self.next_handler.handle_request()

    def real_handler(self) -> None:
        # This function gets implemented in the concrete classes
        raise NotImplementedError("You should implement this!")

    def add_handler(self, new_handler: Self) -> Self:
        self.next_handler = new_handler
        return self.next_handler  # Allows for chaining .add_handler().add_handler()...
