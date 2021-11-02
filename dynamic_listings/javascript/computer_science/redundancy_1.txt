class List{
    constructor(){
        this.nodeList = null;
    }
    // ...
    getLength(){
        let counter = 0;
        for (const item in this.nodeList){
            counter = counter + 1;
        }
        return counter;
    }
}
