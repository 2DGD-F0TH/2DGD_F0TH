class Singleton:
    __instance = None
    @staticmethod
    def get_instance():
        """ Static access method. """
        if not Singleton.__instance:
            Singleton()
        return Singleton.__instance

    def __init__(self):
        """ Virtually private constructor. """
        if Singleton.__instance:
            raise Exception("This class is a singleton, cannot instantiate")
        else:
            Singleton.__instance = self
