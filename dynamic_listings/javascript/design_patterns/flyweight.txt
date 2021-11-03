class Common{
    // Contains the common data for a 3D Object to be replicated
    constructor(){
        this.mesh = new Mesh();
        this.texture = new Texture();
    }
}

class FlyWeight{
    // Contains only the necessary data to create an instance of the item
    constructor(common_ptr){
        this.common_pointer = common_ptr;
        this.position = new Vector2();
        this.scale_factor = 1;
    }
}
