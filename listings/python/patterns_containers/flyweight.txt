class Common:
    # Contains the common data for a 3D Object to be replicated
    def __init__(self, mesh, texture):
        self._mesh = mesh
        self._texture = texture


class FlyWeight:
    # Contains only the necessary data to create an instance of the item
    def __init__(self, common, position, scale_factor):
        self._common_pointer = common
        self._position = position
        self._scale_factor = scale_factor
