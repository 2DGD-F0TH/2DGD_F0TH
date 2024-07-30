from typing import Self

class Singleton:

    @staticmethod
    def get_instance() -> Self:
        return Singleton.__instance

    def __init__(self):
        """
        We make it so the constructor is unusable from outside
        """
        if Singleton.__instance:
            raise RuntimeError("This class is a singleton, cannot instantiate")
        Singleton.__instance = self

    __instance = __init__()
