class Polygon:
    vertices: list[Point] = []

    def calculate_bounding_box(self) -> Rectangle:
        # This function calculates the bounding box
        # -------------------------
        # First we create and bootstrap the variables
        xmin: int = self.vertices[0].x
        xmax: int = self.vertices[0].x
        # ...
        # see the bounding box algorithm for the full version
        # ...
        # We build our bounding box
        boundingBox = Rectangle.from_points(A, C)
        # and return it
        return boundingBox

    def do_fanning(self) -> list[Triangle]:
        """
        This function iterates over the vertices and returns
        an array of triangles corresponding to the "fan triangulation"
        """
        # We fix the "base" of the fan on the first vertex
        root_vertex: Point = self.vertices[0]
        temp_triangles: list[Triangle] = []
        # Now we iterate through all the other vertices
        for j in range(2, len(self.vertices)):
            # j goes from the third vertex, to the last
            # j - 1 goes from the second to the second to last
            temp_triangles.append(Triangle.from_points(root_vertex, j - 1, j))
        # In the end, we will have the triangles array, we can just return it
        return temp_triangles
