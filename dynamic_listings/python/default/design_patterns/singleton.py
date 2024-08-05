class Singleton:
    __instance = None

    @staticmethod
    def get_instance():
        """ Static access method. """
        if not Singleton.__instance:
            Singleton()
        return Singleton.__instance

    def __init__(self) -> None:
        """ Virtually private constructor. """
        if Singleton.__instance:
            raise RuntimeError(
                "This class is a singleton, cannot instantiate"
            )
        Singleton.__instance = self
