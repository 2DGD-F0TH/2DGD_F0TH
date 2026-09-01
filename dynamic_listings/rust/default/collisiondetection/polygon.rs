struct Polygon {
    vertices: Vec<Point>,
}

impl Polygon {
    pub fn calculate_bounding_box(&self) -> Rectangle {
        // This function calculates the bounding box
        // -------------------------
        // First we create and bootstrap the variables
        let xmin = self.vertices[0].x;
        let xmax = self.vertices[0].x;
        /*
         * ...
         * see the bounding box algorithm for the full version
         * ...
         */
        // We build our bounding box
        Rectangle::from_points(&a, &c)
    }

    pub fn do_fanning(&self) -> Vec<Triangle> {
        /*
         * This function iterates over the vertices and returns
         * an array of triangles corresponding to the "fan triangulation"
         */
        // We fix the "base" of the fan on the first vertex
        let root_vertex = &self.vertices[0];
        let mut temp_triangles = Vec::new();
        // Now we iterate through all the other vertices
        for x in 0..self.vertices.len() {
            // x goes from the third vertex, to the last
            // x - 1 goes from the second to the second to last
            temp_triangles.push(Triangle::from_points(
                &root_vertex,
                &self.vertices[x - 1],
                &self.vertices[x],
            ));
        }
        // In the end, we will have the triangles array, we can just return it
        temp_triangles
    }
}
