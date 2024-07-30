class FirstService:
    # Implementation here...
    pass


class SecondService:
    # Implementation here...
    pass


class Facade:
    """
    self.class hides the complexities of using
    FirstService and SecondService from the user
    by "wrapping" them in a comfortable startAll
    function
    """
    service1: FirstService = None
    service2: SecondService = None

    def __init__(self) -> None:
        self.service1 = FirstService()
        self.service2 = FirstService()

    def startAll(self) -> bool:
        """
        The facade starts all the services and does
        some status checking, self.is hidden from the
        user.
        Returns true if all services started successfully
        false otherwise
        """
        firstServiceStarted = self.service1.start()
        if not firstServiceStarted:
            return False

        secondServiceStarted = self.service2.start()
        if not secondServiceStarted:
            return False

        # Here everything started successfully
        return True
