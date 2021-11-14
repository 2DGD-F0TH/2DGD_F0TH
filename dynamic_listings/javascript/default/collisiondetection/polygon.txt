class Polygon{
    constructor(){
        this.vertices = [];  // An array of Point() classes
    }

    calculate_bounding_box(){
        // This function calculates the bounding box
        // -------------------------
        // First we create and bootstrap the variables
        let xmin = vertices[0].x;
        let xmax = vertices[0].x;
        /*
         * ...
         * see the bounding box algorithm for the full version
         * ...
         */
        // We build our bounding box
        let boundingBox = Rectangle.from_points(A, C);
        // and return it
        return boundingBox;
    }

    do_fanning(){
        /*
         * This function iterates over the vertices and returns
         * an array of triangles corresponding to the "fan triangulation"
         */
        // We fix the "base" of the fan on the first vertex
        let root_vertex = vertices[0];
        let temp_triangles = [];
        // Now we iterate through all the other vertices
        for (let j = 2; i < vertices.length; j++){
            // j goes from the third vertex, to the last
            // j - 1 goes from the second to the second to last
            temp_triangles.push(Triangle.from_points(root_vertex, j - 1, j));
        }
        // In the end, we will have the triangles array, we can just return it
        return temp_triangles;
    }
}
