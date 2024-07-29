class Common:
    # Contains the common data for a 3D Object to be replicated
    def __init__(self, mesh: Mesh, texture: Texture):
        self._mesh = mesh
        self._texture = texture


class FlyWeight:
    # Contains only the necessary data to create an instance of the item
    def __init__(self, common: Common, position: Vector, scale_factor: float):
        self._common_pointer = common
        self._position = position
        self._scale_factor = scale_factor
