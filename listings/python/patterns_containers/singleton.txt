class Singleton:

    @staticmethod
    def getInstance():
        return Singleton.__instance

    def __init__(self):
        """
        We make it so the constructor is unusable from outside
        """
        if Singleton.__instance is not None:
            raise Exception("This class is a singleton, cannot instantiate")
        else:
            Singleton.__instance = self

    __instance = __init__()
