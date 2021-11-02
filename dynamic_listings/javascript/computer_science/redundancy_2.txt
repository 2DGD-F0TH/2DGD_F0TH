class List{
    constructor(){
        this.nodeList = null;
        this.length = 0;
    }
    // ...
    getLength(){
        return length;
    }

    addItem(node){
        // ... Normal operation ...
        // ...
        // We update our length counter
        this.length = this.length + 1;
    }

    removeItem(node){
        // ... Normal removal operation ...
        // ...
        // We update our length counter
        this.length = this.length - 1;
    }

    clear(){
        // ... Normal clear operation ...
        // ...
        // We clear the length too
        this.length = 0;
    }

    // ...
}
