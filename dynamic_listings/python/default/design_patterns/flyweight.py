class Common:
    # Contains the common data for a 3D Object to be replicated
    def __init__(self, mesh: Mesh, texture: Texture):
        self._mesh: Mesh = mesh
        self._texture: Texture = texture


class FlyWeight:
    # Contains only the necessary data to create an instance of the item
    def __init__(self, common: Common, position: Vector2, scale_factor: float):
        self._common_pointer: Common = common
        self._position: Vector2 = position
        self._scale_factor: float = scale_factor
