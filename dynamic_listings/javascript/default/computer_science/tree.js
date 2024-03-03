class Node{
    /*
     * This is an example of a simple node structure for a tree.
     * It can be used as root or any other node
     */

    constructor(String value){
        this.content = value;
        this.left = null;
        this.right = null;
    }
}


function build_example_tree(){
    // Let's build the example tree; starting with the nodes
    let A = Node("A");
    let B = Node("B");
    let C = Node("C");
    let D = Node("D");
    let E = Node("E");
    let F = Node("F");
    let G = Node("G");
    let H = Node("H");
    let I = Node("I");
    // Now we connect the various components (the edges)
    B.left = A;
    B.right = C;
    F.left = E;
    D.left = B;
    D.right = F;
    H.right = I;
    G.left = D;
    G.right = H;
    // The tree is ready to be used, let's return the root (G)
    return G;
}
