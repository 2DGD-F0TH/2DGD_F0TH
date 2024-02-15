class Handler:
    # This is the handler abstract/class interface that the sender connects to
    next = None  # The next handler in the chain

    def handle_request(self):
        if condition:
            # In case I can handle this request
            return self.real_handler()

        if self.next is not None:
            self.next.handle_request()

    def real_handler(self):
        # This function gets implemented in the concrete classes
        raise NotImplementedError("You should implement this!")

    def add_handler(self, new_handler):
        self.next = new_handler
        return self.next;  # Allows for chaining .add_handler().add_handler()...
